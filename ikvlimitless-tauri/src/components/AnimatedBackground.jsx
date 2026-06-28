import React, { useEffect, useRef } from 'react'
import { getSavedBgTheme } from '../utils/theme'

export default function AnimatedBackground() {
  const canvasRef = useRef(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    let width, height
    let animationFrameId

    let mouse = { x: -1000, y: -1000, radius: 180, isMoving: false }
    let currentBgTheme = getSavedBgTheme()

    let mouseTimeout;

    const resize = () => {
      width = window.innerWidth
      height = window.innerHeight
      canvas.width = width
      canvas.height = height
      init() // Re-initialize particles on resize
    }

    window.addEventListener('resize', resize)

    const handleMouseMove = (e) => {
      mouse.x = e.clientX
      mouse.y = e.clientY
      mouse.isMoving = true
      clearTimeout(mouseTimeout)
      mouseTimeout = setTimeout(() => { mouse.isMoving = false }, 100)
    }
    const handleMouseLeave = () => {
      mouse.x = -1000
      mouse.y = -1000
      mouse.isMoving = false
    }
    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseout', handleMouseLeave)

    function hexToRgba(hex, alpha) {
      if (!hex) return `rgba(255, 255, 255, ${alpha})`
      hex = hex.replace('#', '')
      if(hex.length === 3) {
         hex = hex[0]+hex[0]+hex[1]+hex[1]+hex[2]+hex[2]
      }
      let r = parseInt(hex.substring(0, 2), 16) || 255
      let g = parseInt(hex.substring(2, 4), 16) || 255
      let b = parseInt(hex.substring(4, 6), 16) || 255
      return `rgba(${r}, ${g}, ${b}, ${alpha})`
    }

    let currentAccent = getComputedStyle(document.documentElement).getPropertyValue('--accent').trim() || '#c27a51'

    let colors = [
      hexToRgba(currentAccent, 0.9),
      hexToRgba(currentAccent, 0.5),
      'rgba(255, 255, 255, 0.8)'
    ]

    const handleThemeChange = (e) => {
      currentAccent = e.detail.accent
      updateColors()
    }
    
    const handleBgThemeChange = (e) => {
      currentBgTheme = e.detail
      init()
    }

    const updateColors = () => {
      colors = [
        hexToRgba(currentAccent, 0.9),
        hexToRgba(currentAccent, 0.5),
        'rgba(255, 255, 255, 0.8)'
      ]
      if (particleArray.length > 0) {
        particleArray.forEach(p => {
          p.color = colors[Math.floor(Math.random() * colors.length)]
        })
      }
    }

    window.addEventListener('theme_changed', handleThemeChange)
    window.addEventListener('bg_theme_changed', handleBgThemeChange)

    class Particle {
      constructor() {
        this.x = Math.random() * width
        this.y = Math.random() * height
        // Base sizes depending on theme
        if (currentBgTheme === 'fireflies') {
           this.size = Math.random() * 3 + 1.5
        } else if (currentBgTheme === 'orbit') {
           this.size = Math.random() * 2 + 1
        } else if (currentBgTheme === 'supernova') {
           this.size = Math.random() * 2 + 0.5
        } else {
           this.size = Math.random() * 2 + 1
        }
        
        this.baseX = this.x
        this.baseY = this.y
        this.prevX = this.x
        this.prevY = this.y
        this.density = (Math.random() * 30) + 1
        this.color = colors[Math.floor(Math.random() * colors.length)]
        this.vx = (Math.random() - 0.5) * 1.5
        this.vy = (Math.random() - 0.5) * 1.5
        
        // Orbit specific
        this.angle = Math.random() * Math.PI * 2
        this.orbitRadius = Math.random() * 150 + 50
        this.orbitSpeed = (Math.random() * 0.02) + 0.005
        this.isOrbiting = false
        
        // Matrix specific
        this.matrixSpeed = Math.random() * 3 + 2
      }

      draw() {
        if (currentBgTheme === 'matrix' || currentBgTheme === 'supernova') {
           ctx.beginPath()
           ctx.moveTo(this.prevX, this.prevY)
           ctx.lineTo(this.x, this.y)
           ctx.strokeStyle = this.color
           ctx.lineWidth = this.size
           ctx.shadowBlur = currentBgTheme === 'supernova' ? 10 : 5
           ctx.shadowColor = this.color
           ctx.stroke()
           ctx.shadowBlur = 0
           return;
        }

        ctx.beginPath()
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
        ctx.closePath()
        ctx.fillStyle = this.color
        
        ctx.shadowBlur = currentBgTheme === 'fireflies' ? 15 : 10
        ctx.shadowColor = this.color
        ctx.fill()
        ctx.shadowBlur = 0
      }

      update() {
        this.prevX = this.x;
        this.prevY = this.y;

        if (currentBgTheme === 'network') {
          this.x += this.vx
          this.y += this.vy

          if (this.x < 0 || this.x > width) this.vx = -this.vx
          if (this.y < 0 || this.y > height) this.vy = -this.vy

          if (mouse.x > 0 && mouse.y > 0) {
            let dx = mouse.x - this.x
            let dy = mouse.y - this.y
            let distance = Math.sqrt(dx * dx + dy * dy)
            if (distance < mouse.radius) {
              let forceDirectionX = dx / distance
              let forceDirectionY = dy / distance
              let force = (mouse.radius - distance) / mouse.radius
              this.x -= forceDirectionX * force * this.density * 0.8
              this.y -= forceDirectionY * force * this.density * 0.8
            }
          }
        } 
        else if (currentBgTheme === 'orbit') {
          let dx = mouse.x - this.x
          let dy = mouse.y - this.y
          let distance = Math.sqrt(dx * dx + dy * dy)

          if (mouse.x > 0 && distance < 300) {
             this.isOrbiting = true
             // Smoothly transition into orbit
             this.angle += this.orbitSpeed
             let targetX = mouse.x + Math.cos(this.angle) * this.orbitRadius
             let targetY = mouse.y + Math.sin(this.angle) * this.orbitRadius
             this.x += (targetX - this.x) * 0.05
             this.y += (targetY - this.y) * 0.05
          } else {
             this.isOrbiting = false
             this.x += this.vx * 0.5
             this.y += this.vy * 0.5
             if (this.x < 0 || this.x > width) this.vx = -this.vx
             if (this.y < 0 || this.y > height) this.vy = -this.vy
          }
        }
        else if (currentBgTheme === 'fireflies') {
          // Fireflies wander around slowly, but follow mouse lazily with inertia
          this.x += this.vx
          this.y += this.vy
          
          // Random wandering variation
          if (Math.random() < 0.05) {
             this.vx += (Math.random() - 0.5) * 0.5
             this.vy += (Math.random() - 0.5) * 0.5
          }
          
          // Speed limit
          const maxSpeed = 2;
          let speed = Math.sqrt(this.vx * this.vx + this.vy * this.vy);
          if (speed > maxSpeed) {
             this.vx = (this.vx / speed) * maxSpeed;
             this.vy = (this.vy / speed) * maxSpeed;
          }

          if (mouse.x > 0 && mouse.y > 0) {
            let dx = mouse.x - this.x
            let dy = mouse.y - this.y
            let distance = Math.sqrt(dx * dx + dy * dy)
            if (distance < 400 && mouse.isMoving) {
               // gently attract to mouse
               this.vx += (dx / distance) * 0.05
               this.vy += (dy / distance) * 0.05
            } else if (distance < 100) {
               // Too close, scatter playfully
               this.vx -= (dx / distance) * 0.2
               this.vy -= (dy / distance) * 0.2
            }
          }
          
          if (this.x < 0) this.x = width;
          if (this.x > width) this.x = 0;
          if (this.y < 0) this.y = height;
          if (this.y > height) this.y = 0;
        }
        else if (currentBgTheme === 'matrix') {
          this.y += this.matrixSpeed;
          this.x += this.vx;
          
          this.vx *= 0.95; // Yavaşça sağa/sola sapmayı sıfırla
          
          if (mouse.x > 0 && mouse.y > 0) {
            let dx = this.x - mouse.x
            let dy = this.y - mouse.y
            let distance = Math.sqrt(dx * dx + dy * dy)
            if (distance < 150) {
               this.vx += (dx / distance) * 1.5;
            }
          }
          
          if (this.y > height) {
             this.y = -10;
             this.x = Math.random() * width;
             this.prevX = this.x;
             this.prevY = this.y;
             this.vx = 0;
          }
          if (this.x < 0) { this.x = width; this.prevX = width; }
          if (this.x > width) { this.x = 0; this.prevX = 0; }
        }
        else if (currentBgTheme === 'supernova') {
          let targetX = (mouse.x > 0) ? mouse.x : width / 2;
          let targetY = (mouse.y > 0) ? mouse.y : height / 2;
          
          let dx = targetX - this.x;
          let dy = targetY - this.y;
          let distance = Math.sqrt(dx * dx + dy * dy);
          
          if (distance > 10) {
             this.vx += (dx / distance) * 0.4;
             this.vy += (dy / distance) * 0.4;
          }
          
          if (distance < 40) {
             this.vx = -(dx / distance) * (Math.random() * 15 + 10);
             this.vy = -(dy / distance) * (Math.random() * 15 + 10);
          }
          
          this.vx *= 0.94;
          this.vy *= 0.94;
          
          this.x += this.vx;
          this.y += this.vy;
          
          if (this.x < -100 || this.x > width + 100 || this.y < -100 || this.y > height + 100) {
             this.x = targetX + (Math.random() - 0.5) * 5;
             this.y = targetY + (Math.random() - 0.5) * 5;
             this.prevX = this.x;
             this.prevY = this.y;
             this.vx = 0;
             this.vy = 0;
          }
        }
      }
    }

    let particleArray = []
    const init = () => {
      particleArray = []
      let numberOfParticles = 0;
      if (currentBgTheme === 'fireflies') {
         numberOfParticles = Math.floor((width * height) / 15000)
      } else if (currentBgTheme === 'orbit') {
         numberOfParticles = Math.floor((width * height) / 8000)
      } else if (currentBgTheme === 'supernova') {
         numberOfParticles = Math.floor((width * height) / 4000)
      } else {
         numberOfParticles = Math.floor((width * height) / 10000)
      }
      
      for (let i = 0; i < numberOfParticles; i++) {
        particleArray.push(new Particle())
      }
    }

    resize()

    const animate = () => {
      if (currentBgTheme === 'fireflies' || currentBgTheme === 'matrix') {
         ctx.fillStyle = 'rgba(10, 10, 15, 0.15)' 
      } else if (currentBgTheme === 'orbit' || currentBgTheme === 'supernova') {
         ctx.fillStyle = 'rgba(10, 10, 15, 0.3)' 
      } else {
         ctx.fillStyle = 'rgba(10, 10, 15, 0.4)' 
      }
      
      ctx.fillRect(0, 0, width, height)
      
      for (let i = 0; i < particleArray.length; i++) {
        particleArray[i].update()
        particleArray[i].draw()
      }
      
      if (currentBgTheme === 'network') {
         connect()
      } else if (currentBgTheme === 'orbit') {
         connectOrbit()
      }
      
      animationFrameId = requestAnimationFrame(animate)
    }

    const connect = () => {
      let opacityValue = 1
      for (let a = 0; a < particleArray.length; a++) {
        for (let b = a; b < particleArray.length; b++) {
          let dx = particleArray[a].x - particleArray[b].x
          let dy = particleArray[a].y - particleArray[b].y
          let distance = dx * dx + dy * dy

          if (distance < 15000) {
            opacityValue = 1 - (distance / 15000)
            let mouseDist = 999999
            if (mouse.x > 0 && mouse.y > 0) {
              let mdx = particleArray[a].x - mouse.x
              let mdy = particleArray[a].y - mouse.y
              mouseDist = mdx * mdx + mdy * mdy
            }

            let isNearMouse = mouseDist < 25000
            ctx.strokeStyle = isNearMouse 
              ? hexToRgba(currentAccent, opacityValue * 0.8)
              : hexToRgba(currentAccent, opacityValue * 0.15)
              
            ctx.lineWidth = isNearMouse ? 1.5 : 0.8
            ctx.beginPath()
            ctx.moveTo(particleArray[a].x, particleArray[a].y)
            ctx.lineTo(particleArray[b].x, particleArray[b].y)
            ctx.stroke()
          }
        }
      }
    }
    
    const connectOrbit = () => {
       // Only connect particles that are actively orbiting and close
      let opacityValue = 1
      for (let a = 0; a < particleArray.length; a++) {
        if (!particleArray[a].isOrbiting) continue;
        for (let b = a; b < particleArray.length; b++) {
          if (!particleArray[b].isOrbiting) continue;
          let dx = particleArray[a].x - particleArray[b].x
          let dy = particleArray[a].y - particleArray[b].y
          let distance = dx * dx + dy * dy

          if (distance < 8000) {
            opacityValue = 1 - (distance / 8000)
            ctx.strokeStyle = hexToRgba(currentAccent, opacityValue * 0.4)
            ctx.lineWidth = 1.0
            ctx.beginPath()
            ctx.moveTo(particleArray[a].x, particleArray[a].y)
            ctx.lineTo(particleArray[b].x, particleArray[b].y)
            ctx.stroke()
          }
        }
      }
    }

    animate()

    return () => {
      window.removeEventListener('resize', resize)
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseout', handleMouseLeave)
      window.removeEventListener('theme_changed', handleThemeChange)
      window.removeEventListener('bg_theme_changed', handleBgThemeChange)
      cancelAnimationFrame(animationFrameId)
    }
  }, [])

  return (
    <canvas
      ref={canvasRef}
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        pointerEvents: 'none',
        zIndex: 0,
      }}
    />
  )
}

import React, { useEffect, useRef } from 'react'

export default function WaveBackground() {
  const canvasRef = useRef(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    let width, height
    let animationFrameId

    let mouse = { x: 0, y: 0 }
    let targetMouse = { x: 0, y: 0 }

    const resize = () => {
      width = window.innerWidth
      height = window.innerHeight
      canvas.width = width
      canvas.height = height
      
      mouse.x = width / 2
      mouse.y = height / 2
      targetMouse.x = width / 2
      targetMouse.y = height / 2
    }

    window.addEventListener('resize', resize)
    resize()

    const handleMouseMove = (e) => {
      targetMouse.x = e.clientX
      targetMouse.y = e.clientY
    }
    window.addEventListener('mousemove', handleMouseMove)

    let time = 0
    
    // Wave configuration: beautiful purple/blue/orange gradient matching the theme
    const waves = [
      { y: 0.4, length: 0.003, amplitude: 60, speed: 0.012, color: 'rgba(194, 122, 81, 0.08)' }, // Accent (orange)
      { y: 0.5, length: 0.002, amplitude: 80, speed: 0.008, color: 'rgba(139, 92, 246, 0.06)' }, // Purple
      { y: 0.6, length: 0.004, amplitude: 45, speed: 0.015, color: 'rgba(30, 64, 175, 0.06)' },  // Blue
      { y: 0.7, length: 0.005, amplitude: 35, speed: 0.020, color: 'rgba(105, 33, 168, 0.08)' }, // Deep Purple
    ]

    const render = () => {
      time += 1
      
      // Smooth mouse follow
      mouse.x += (targetMouse.x - mouse.x) * 0.05
      mouse.y += (targetMouse.y - mouse.y) * 0.05

      ctx.clearRect(0, 0, width, height)

      waves.forEach((wave, index) => {
        ctx.beginPath()
        ctx.moveTo(0, height)
        ctx.lineTo(0, height * wave.y)

        for (let x = 0; x <= width; x += 10) {
          // Distance from point to mouse
          const dx = x - mouse.x
          // Calculate a localized wave disruption around the mouse X
          const distFactor = Math.exp(-(dx * dx) / 80000)
          
          // Influence of mouse Y on the wave amplitude
          const dy = (mouse.y - height / 2) * 0.3
          
          const y = height * wave.y 
                    + Math.sin(x * wave.length + time * wave.speed) * wave.amplitude 
                    + distFactor * dy * (index % 2 === 0 ? 1 : -1)

          ctx.lineTo(x, y)
        }

        // Fill below the wave
        ctx.lineTo(width, height)
        ctx.closePath()

        ctx.fillStyle = wave.color
        ctx.fill()
        
        // Add subtle stroke on top of the wave
        ctx.lineWidth = 1
        ctx.strokeStyle = wave.color.replace(/0\.0[68]/, '0.2')
        ctx.stroke()
      })
      
      // Add an ambient glow following the mouse
      const glow = ctx.createRadialGradient(mouse.x, mouse.y, 0, mouse.x, mouse.y, 400)
      glow.addColorStop(0, 'rgba(194, 122, 81, 0.07)')
      glow.addColorStop(1, 'rgba(194, 122, 81, 0)')
      ctx.fillStyle = glow
      ctx.beginPath()
      ctx.arc(mouse.x, mouse.y, 400, 0, Math.PI * 2)
      ctx.fill()

      animationFrameId = requestAnimationFrame(render)
    }

    render()

    return () => {
      window.removeEventListener('resize', resize)
      window.removeEventListener('mousemove', handleMouseMove)
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

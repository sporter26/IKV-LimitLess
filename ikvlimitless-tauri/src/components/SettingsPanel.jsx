import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { toast } from '../utils/modal'
import { THEMES, applyTheme, getSavedThemeId, BG_THEMES, applyBgTheme, getSavedBgTheme } from '../utils/theme'

export default function SettingsPanel() {
  const [gamePath, setGamePath] = useState('')
  const [saving, setSaving] = useState(false)
  const [activeTheme, setActiveTheme] = useState(getSavedThemeId())
  const [activeBgTheme, setActiveBgTheme] = useState(getSavedBgTheme())

  useEffect(() => {
    loadGamePath()
  }, [])

  const loadGamePath = async () => {
    try {
      const res = await invoke('get_game_path')
      if (res.success && res.data) setGamePath(res.data)
    } catch (e) {
      console.error(e)
    }
  }

  const handleSelectPath = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      })

      if (selected) {
        setSaving(true)
        const res = await invoke('set_game_path', { path: selected })
        if (res.success) {
          setGamePath(selected)
          toast.success('Oyun yolu kaydedildi')
        } else {
          toast.error(res.error || 'Yol kaydedilemedi')
        }
        setSaving(false)
      }
    } catch (e) {
      toast.error('Dosya seçilirken hata oluştu')
      setSaving(false)
    }
  }

  const handleThemeChange = (themeId) => {
    setActiveTheme(themeId)
    applyTheme(themeId)
  }

  const handleBgThemeChange = (bgThemeId) => {
    setActiveBgTheme(bgThemeId)
    applyBgTheme(bgThemeId)
  }

  const handleLogout = async () => {
    window.location.reload()
  }

  const hasPath = gamePath && gamePath.length > 0

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>

      {/* Theme Selection Card */}
      <div className="glass-card">
        <div className="section-header">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" strokeWidth="2">
            <path d="M12 2.69l5.66 4.2c.2.15.34.36.4.6l2.12 7.54c.06.21.03.44-.09.63l-4.5 7.03c-.14.22-.38.35-.64.35H9.05c-.26 0-.5-.13-.64-.35l-4.5-7.03a1.1 1.1 0 0 1-.09-.63l2.12-7.54c.06-.24.2-.45.4-.6L12 2.69z" />
          </svg>
          <span className="section-title">Tema Rengi</span>
        </div>
        
        <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
          {THEMES.map(theme => (
            <div
              key={theme.id}
              onClick={() => handleThemeChange(theme.id)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                padding: '8px 12px',
                borderRadius: 'var(--radius-sm)',
                background: activeTheme === theme.id ? 'rgba(255,255,255,0.1)' : 'rgba(255,255,255,0.03)',
                border: `1px solid ${activeTheme === theme.id ? theme.accent : 'rgba(255,255,255,0.05)'}`,
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
            >
              <div style={{
                width: '16px',
                height: '16px',
                borderRadius: '50%',
                background: theme.accent,
                boxShadow: activeTheme === theme.id ? `0 0 12px ${theme.glow}` : 'none'
              }} />
              <span style={{ fontSize: '13px', color: activeTheme === theme.id ? '#fff' : 'var(--text-secondary)' }}>
                {theme.name}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* BG Theme Selection Card */}
      <div className="glass-card">
        <div className="section-header">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" strokeWidth="2">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
          <span className="section-title">Arkaplan Animasyonu</span>
        </div>
        
        <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
          {BG_THEMES.map(bg => (
            <div
              key={bg.id}
              onClick={() => handleBgThemeChange(bg.id)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                padding: '8px 12px',
                borderRadius: 'var(--radius-sm)',
                background: activeBgTheme === bg.id ? 'rgba(255,255,255,0.1)' : 'rgba(255,255,255,0.03)',
                border: `1px solid ${activeBgTheme === bg.id ? 'var(--accent)' : 'rgba(255,255,255,0.05)'}`,
                cursor: 'pointer',
                transition: 'all 0.2s',
              }}
            >
              <span style={{ fontSize: '13px', color: activeBgTheme === bg.id ? '#fff' : 'var(--text-secondary)' }}>
                {bg.name}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Game Path Card */}
      <div className="glass-card">
        <div className="section-header">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" strokeWidth="2">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.07 4.93L16.24 7.76M4.93 19.07l2.83-2.83M21 12h-4M7 12H3M19.07 19.07l-2.83-2.83M7.76 7.76L4.93 4.93" />
          </svg>
          <span className="section-title">Oyun Ayarları</span>
        </div>

        <div style={{ marginBottom: '16px' }}>
          <label className="field-label">
            Oyun Klasörü (Istanbul Kiyamet Vakti)
          </label>

          <div style={{ display: 'flex', gap: '8px' }}>
            <div style={{
              flex: 1,
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              background: 'rgba(255,255,255,0.03)',
              border: `1px solid ${hasPath ? 'rgba(34,197,94,0.3)' : 'var(--border)'}`,
              borderRadius: 'var(--radius-sm)',
              padding: '10px 14px',
              transition: 'border-color 0.2s',
            }}>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke={hasPath ? 'var(--green)' : 'var(--text-muted)'} strokeWidth="2">
                <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" />
                <polyline points="13 2 13 9 20 9" />
              </svg>
              <span style={{
                fontSize: 12,
                color: hasPath ? 'var(--text-secondary)' : 'var(--text-muted)',
                fontFamily: 'monospace',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                whiteSpace: 'nowrap',
              }}>
                {hasPath ? gamePath : 'Henüz seçilmedi...'}
              </span>
            </div>

            <button
              className="btn btn-ghost"
              onClick={handleSelectPath}
              disabled={saving}
              style={{ flexShrink: 0 }}
            >
              {saving ? (
                <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                </svg>
              ) : (
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
                </svg>
              )}
              Gözat
            </button>
          </div>

          {hasPath && (
            <p style={{ marginTop: '8px', fontSize: 11, color: 'var(--green)', display: 'flex', alignItems: 'center', gap: '4px' }}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              Oyun yolu kaydedildi
            </p>
          )}
        </div>

        {/* Game path explanation */}
        <div style={{
          padding: '12px 14px',
          borderRadius: 'var(--radius-sm)',
          background: 'rgba(99,102,241,0.05)',
          border: '1px solid rgba(99,102,241,0.15)',
        }}>
          <p style={{ fontSize: 12, color: 'var(--text-secondary)', lineHeight: 1.6 }}>
            <span style={{ color: 'var(--accent)', fontWeight: 600 }}>💡 Bilgi: </span>
            Lütfen C:\Sobee\İstanbul Kıyamet Vakti (veya oyunu kurduğunuz farklı bir dizin) klasörünü seçtiğinizden emin olun. Seçtiğiniz klasörün içinde "istanbul.exe" bulunmalıdır.
          </p>
        </div>
      </div>




      {/* Logout Card */}
      <div className="glass-card" style={{ marginTop: 'auto', border: '1px solid rgba(239, 68, 68, 0.2)' }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div>
            <div style={{ fontSize: 14, fontWeight: 600, color: 'var(--red)' }}>Oturumu Kapat</div>
            <div style={{ fontSize: 12, color: 'var(--text-muted)', marginTop: 4 }}>
              Uygulamadan güvenli bir şekilde çıkış yapın
            </div>
          </div>
          <button
            onClick={handleLogout}
            style={{
              background: 'rgba(239, 68, 68, 0.1)',
              color: 'var(--red)',
              border: '1px solid rgba(239, 68, 68, 0.3)',
              padding: '8px 16px',
              borderRadius: '12px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
              transition: 'all 0.2s ease',
            }}
            onMouseOver={(e) => {
              e.currentTarget.style.background = 'rgba(239, 68, 68, 0.2)';
            }}
            onMouseOut={(e) => {
              e.currentTarget.style.background = 'rgba(239, 68, 68, 0.1)';
            }}
          >
            Çıkış Yap
          </button>
        </div>
      </div>
    </div>
  )
}

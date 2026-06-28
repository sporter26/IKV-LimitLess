import { useState } from 'react'

export default function AccountList({ accounts, loading, onLaunch, onRemove }) {
  if (loading) {
    return (
      <div className="glass-card">
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px', color: 'var(--text-muted)', padding: '20px' }}>
          <svg className="spinner" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
          </svg>
          Hesaplar yükleniyor...
        </div>
      </div>
    )
  }

  return (
    <div className="glass-card">
      <div className="section-header">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--text-secondary)" strokeWidth="2">
          <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" />
          <circle cx="9" cy="7" r="4" />
          <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
          <path d="M16 3.13a4 4 0 0 1 0 7.75" />
        </svg>
        <span className="section-title">Kayıtlı Hesaplar</span>
        <span style={{ marginLeft: 'auto' }} className="badge badge-gray">
          {accounts.length} toplam
        </span>
      </div>

      {accounts.length === 0 ? (
        <div className="empty-state">
          <svg className="empty-state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
            <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
            <circle cx="9" cy="7" r="4" />
            <line x1="19" y1="8" x2="19" y2="14" />
            <line x1="22" y1="11" x2="16" y2="11" />
          </svg>
          <p style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Henüz hesap eklenmemiş</p>
          <p style={{ fontSize: 11, color: 'var(--text-muted)' }}>Yukarıdaki formu kullanarak hesap ekleyin</p>
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          {accounts.map((account, index) => (
            <AccountRow
              key={account.id}
              account={account}
              index={index}
              onLaunch={onLaunch}
              onRemove={onRemove}
            />
          ))}
        </div>
      )}
    </div>
  )
}

function AccountRow({ account, index, onLaunch, onRemove }) {
  const [removing, setRemoving] = useState(false)
  const [launching, setLaunching] = useState(false)

  const handleLaunch = async () => {
    setLaunching(true)
    await onLaunch(account.id)
    setLaunching(false)
  }

  const handleRemove = async () => {
    setRemoving(true)
    await onRemove(account.id)
    setRemoving(false)
  }

  // Hesap için renk üret (id'ye göre)
  const hue = account.id.split('').reduce((acc, c) => acc + c.charCodeAt(0), 0) % 360

  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: '12px',
        padding: '12px 14px',
        borderRadius: 'var(--radius-md)',
        background: 'rgba(255,255,255,0.02)',
        border: '1px solid var(--border)',
        transition: 'all 0.2s ease',
        animationDelay: `${index * 0.05}s`,
      }}
      className="animate-fade-in"
      onMouseEnter={e => e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)'}
      onMouseLeave={e => e.currentTarget.style.borderColor = 'var(--border)'}
    >
      {/* Avatar */}
      <div style={{
        width: 38,
        height: 38,
        borderRadius: '10px',
        background: `hsl(${hue}deg 45% 25%)`,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        fontSize: 15,
        fontWeight: 700,
        color: `hsl(${hue}deg 70% 75%)`,
        flexShrink: 0,
        fontFamily: "'Space Grotesk', sans-serif",
        border: `1px solid hsl(${hue}deg 40% 35%)`,
      }}>
        {account.username.charAt(0).toUpperCase()}
      </div>

      {/* Info */}
      <div style={{ flex: 1, minWidth: 0 }}>
        <p style={{
          fontSize: 13,
          fontWeight: 600,
          color: 'var(--text-primary)',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          whiteSpace: 'nowrap',
          marginBottom: '2px',
        }}>
          {account.username}
        </p>
        <p style={{
          fontSize: 11,
          color: 'var(--text-muted)',
          fontFamily: 'monospace',
        }}>
          {account.server}:{account.server_port}
        </p>
      </div>

      {/* Status badges */}
      <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
        {account.farm_mode && (
          <span className="badge badge-yellow" title="Farm modu aktif">🌾</span>
        )}
        {account.boss_mode && (
          <span className="badge badge-purple" title="Boss modu aktif">⚔️</span>
        )}
        <span className={account.is_running ? 'badge badge-green' : 'badge badge-gray'}>
          {account.is_running && (
            <span className="pulse-dot" style={{
              width: 5,
              height: 5,
              borderRadius: '50%',
              background: 'var(--green)',
              display: 'inline-block',
              flexShrink: 0,
            }} />
          )}
          {account.is_running ? 'Çalışıyor' : 'Durdu'}
        </span>
      </div>

      {/* Actions */}
      <div style={{ display: 'flex', gap: '6px', flexShrink: 0 }}>
        <button
          className="btn btn-success btn-icon"
          onClick={handleLaunch}
          disabled={launching}
          title="Giriş Penceresi Aç"
        >
          {launching ? (
            <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
            </svg>
          ) : (
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <polygon points="5 3 19 12 5 21 5 3" />
            </svg>
          )}
        </button>
        <button
          className="btn btn-danger btn-icon"
          onClick={handleRemove}
          disabled={removing}
          title="Hesabı Sil"
        >
          {removing ? (
            <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
            </svg>
          ) : (
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <polyline points="3 6 5 6 21 6" />
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
            </svg>
          )}
        </button>
      </div>
    </div>
  )
}

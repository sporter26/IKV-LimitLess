import { useState, useEffect, useRef, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { toast } from '../utils/modal'
import SettingsPanel from '../components/SettingsPanel'
import logoImg from '../assets/logo.jpg'

// Nickname storage (localStorage)
const getNicknames = () => { try { return JSON.parse(localStorage.getItem('ll_nicknames') || '{}') } catch { return {} } }
const saveNicknames = (nicks) => localStorage.setItem('ll_nicknames', JSON.stringify(nicks))

// ─── ICONS ───────────────────────────────────────────────────────────────────
const IconRun = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="5" r="2" /><path d="M12 7v8" /><path d="M7 12l5 3 5-3" /><path d="M7 17l2.5 2M17 17l-2.5 2" />
  </svg>
)
const IconBot = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <rect x="3" y="11" width="18" height="10" rx="2" /><circle cx="12" cy="5" r="2" />
    <path d="M12 7v4" /><circle cx="8" cy="16" r="1" fill="currentColor" /><circle cx="16" cy="16" r="1" fill="currentColor" />
  </svg>
)

const IconSettings = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
  </svg>
)
const IconRefresh = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <polyline points="23 4 23 10 17 10" /><polyline points="1 20 1 14 7 14" />
    <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
  </svg>
)
const IconPower = () => (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <path d="M18.36 6.64a9 9 0 1 1-12.73 0" /><line x1="12" y1="2" x2="12" y2="12" />
  </svg>
)
const IconStar = ({ filled }) => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill={filled ? '#fbbf24' : 'none'} stroke={filled ? '#fbbf24' : 'currentColor'} strokeWidth="2">
    <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
  </svg>
)
const IconWindowSize = ({ size = 'small' }) => {
  if (size === 'small') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="7" y="7" width="10" height="10" rx="1" /></svg>
  } else if (size === 'medium') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="4" y="4" width="16" height="16" rx="1" /></svg>
  } else if (size === 'large') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="2" y="2" width="20" height="20" rx="1" /></svg>
  } else if (size === '2k_4') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="5" y="7" width="14" height="10" rx="1" /></svg>
  } else if (size === '2k_2') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="6" y="3" width="12" height="18" rx="1" /></svg>
  } else if (size === '2k_full') {
    return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="1" y="3" width="22" height="18" rx="1" /></svg>
  }
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><rect x="7" y="7" width="10" height="10" rx="1" /></svg>
}
const IconSort = () => (
  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
    <line x1="3" y1="6" x2="21" y2="6" /><line x1="8" y1="12" x2="21" y2="12" /><line x1="13" y1="18" x2="21" y2="18" />
  </svg>
)




const SERVERS = [
  { label: 'Eminönü', value: '93.155.105.236', port: 27201 },
  { label: 'Beyazköşk', value: '93.155.105.236', port: 27205 },
  { label: 'Galata', value: '93.155.105.236', port: 27206 },
  { label: 'Karaköy', value: '93.155.105.236', port: 27207 },
  { label: 'Meran', value: '93.155.105.236', port: 27208 },
  { label: "Bab-ı Ali", value: '93.155.105.236', port: 27209 },
]

// ─── Dashboard ────────────────────────────────────────────────────────────────
export default function Dashboard({ accounts, onAccountsChange, loading }) {
  const [activeTab, setActiveTab] = useState('accounts')
  const [addLoading, setAddLoading] = useState(false)
  const [launchAllLoading, setLaunchAllLoading] = useState(false)
  const [showModal, setShowModal] = useState(false)
  const [showProfilePopup, setShowProfilePopup] = useState(false)
  const [profile, setProfile] = useState(null)
  const [appVersion, setAppVersion] = useState('')
  const [editingAccountId, setEditingAccountId] = useState(null)
  const [nicknames, setNicknames] = useState(getNicknames)
  const [favorites, setFavorites] = useState(() => {
    try { return JSON.parse(localStorage.getItem('ll_favorites') || '[]') } catch { return [] }
  })
  const [windowSizes, setWindowSizes] = useState(() => {
    try { return JSON.parse(localStorage.getItem('ll_windowsizes') || '{}') } catch { return {} }
  })
  const [customSizes, setCustomSizes] = useState(() => {
    try { return JSON.parse(localStorage.getItem('ll_customsizes') || '{}') } catch { return {} }
  })
  const [showFavoritesOnly, setShowFavoritesOnly] = useState(false)
  const [serverFilter, setServerFilter] = useState('all')
  const [selected, setSelected] = useState([])
  const [dragState, setDragState] = useState({ dragging: null, over: null })
  const [customOrder, setCustomOrder] = useState(() => {
    try { return JSON.parse(localStorage.getItem('ll_order') || '[]') } catch { return [] }
  })
  const [showPassword, setShowPassword] = useState(false)
  const [hideInfo, setHideInfo] = useState(false)

  const [newForm, setNewForm] = useState({
    nickname: '', username: '', password: '',
    server: '93.155.105.236', server_port: 27201,
  })
  const [openPopupAccountId, setOpenPopupAccountId] = useState(null)


  // Sync nicknames from localStorage when accounts update (e.g., after cloud sync)
  useEffect(() => {
    setNicknames(getNicknames())
  }, [accounts])

  // Listen to background PID monitor changes
  useEffect(() => {
    const unlisten = listen('accounts_changed', () => {
      onAccountsChange()
    })

    // Her 15 saniyede bir yüzeysel hesap yenilemesi
    const intervalId = setInterval(onAccountsChange, 15000);

    return () => {
      unlisten.then(f => f())
      clearInterval(intervalId)
    }
  }, [onAccountsChange])

  // Reset stale running states when app starts
  useState(() => {
    invoke('reset_all_running').catch(() => { })
  })

  const toggleFavorite = (id) => {
    const next = favorites.includes(id) ? favorites.filter(f => f !== id) : [...favorites, id]
    setFavorites(next)
    localStorage.setItem('ll_favorites', JSON.stringify(next))
  }

  const toggleSelect = (id) =>
    setSelected(s => s.includes(id) ? s.filter(x => x !== id) : [...s, id])

  const changeWindowSize = (id, size) => {
    const newSizes = { ...windowSizes, [id]: size }
    setWindowSizes(newSizes)
    localStorage.setItem('ll_windowsizes', JSON.stringify(newSizes))
    
    if (size.startsWith('custom_')) {
      const newCustomSizes = { ...customSizes, [id]: size }
      setCustomSizes(newCustomSizes)
      localStorage.setItem('ll_customsizes', JSON.stringify(newCustomSizes))
    }
  }

  const handleSaveAccount = async () => {
    if (!newForm.username || !newForm.password) {
      toast.error('Kullanıcı adı ve şifre gerekli')
      return
    }
    setAddLoading(true)
    try {
      if (editingAccountId) {
        const response = await invoke('update_account', {
          req: { id: editingAccountId, username: newForm.username, password: newForm.password, server: newForm.server, server_port: newForm.server_port }
        })
        if (response.success) {
          if (newForm.nickname) {
            const updated = { ...getNicknames(), [editingAccountId]: newForm.nickname }
            saveNicknames(updated)
            setNicknames(updated)
          }



          toast.success('Hesap güncellendi')
          setNewForm({ nickname: '', username: '', password: '', server: '93.155.105.236', server_port: 27201 })
          setShowModal(false)
          setEditingAccountId(null)
          onAccountsChange()
        } else toast.error(response.error || 'Güncellenemedi')
      } else {
        const response = await invoke('add_account', {
          req: { username: newForm.username, password: newForm.password, server: newForm.server, server_port: newForm.server_port }
        })
        if (response.success && response.data) {
          const accountId = response.data.id;
          if (newForm.nickname) {
            const updated = { ...getNicknames(), [accountId]: newForm.nickname }
            saveNicknames(updated)
            setNicknames(updated)
          }



          toast.success(`"${newForm.nickname || newForm.username}" eklendi`)
          setNewForm({ nickname: '', username: '', password: '', server: '93.155.105.236', server_port: 27201 })
          setShowModal(false)
          onAccountsChange()
        } else {
          toast.error(response.error || 'Hesap eklenirken hata oluştu')
        }
      }
    } catch { toast.error('Bağlantı hatası') }
    finally { setAddLoading(false) }
  }

  const handleLogout = async () => {
    window.location.reload()
  }

  const handleLaunchGame = async (accountId) => {
    try {
      const acc = accounts.find(a => a.id === accountId);
      const window_size = windowSizes[accountId] || 'small';

      // Spoofer ayarını localStorage'dan hızlı al, ağ isteği bekleme
      let spoofer_active = true;
      try {
        const cached = localStorage.getItem('ll_settings');
        if (cached) {
          const s = JSON.parse(cached);
          spoofer_active = s.spoofer_active !== undefined ? s.spoofer_active : true;
        }
      } catch { }

      toast.info('Oyun başlatılıyor...')
      const res = await invoke('launch_game_direct', { account_id: accountId, spoofer_active, window_size, bot_token: "local_bypass" })
      if (res.success) {
        toast.success('Giriş yapılıyor!');
        onAccountsChange()
      }
      else toast.error(res.error || 'Başlatma başarısız')
    } catch (e) { toast.error('Hata: ' + e) }
  }

  const handleRemoveAccount = async (accountId) => {
    try {
      const acc = accounts.find(a => a.id === accountId);
      const res = await invoke('remove_account', { account_id: accountId })
      if (res.success) {
        toast.success('Hesap silindi');

        onAccountsChange()
      }
      else toast.error(res.error || 'Silinemedi')
    } catch { toast.error('Hata oluştu') }
  }

  const handleStopAccount = async (accountId) => {
    try {
      const res = await invoke('stop_account', { account_id: accountId })
      if (res.success) { toast.success('Hesap durduruldu'); onAccountsChange() }
      else toast.error(res.error || 'Durdurulamadı')
    } catch { toast.error('Hata oluştu') }
  }

  const handleNicknameChange = (accountId, newNick) => {
    const updated = { ...getNicknames(), [accountId]: newNick }
    saveNicknames(updated)
    setNicknames(updated)
  }

  const handleLaunchAll = async () => {
    if (!accounts.length) { toast.error('Hesap yok'); return }

    setLaunchAllLoading(true)
    try {
      let spoofer_active = true;
      try {
        const cached = localStorage.getItem('ll_settings');
        if (cached) {
          const s = JSON.parse(cached);
          spoofer_active = s.spoofer_active !== undefined ? s.spoofer_active : true;
        }
      } catch {}

      const window_sizes_map = {};
      accounts.forEach(a => {
        window_sizes_map[a.id] = windowSizes[a.id] || 'small';
      });

      const res = await invoke('launch_all_accounts', {
        spoofer_active: spoofer_active,
        window_sizes: window_sizes_map,
        bot_token: "local_bypass"
      });
      if (res.success) { toast.success(`${res.data?.data || 0} hesap başlatıldı`); onAccountsChange() }
      else toast.error(res.error || 'Başlatma hatası')
    } catch { toast.error('Hata oluştu') }
    finally { setLaunchAllLoading(false) }
  }

  const handleStopAll = async () => {
    try {
      const res = await invoke('stop_all_accounts')
      if (res.success) { toast.success('Tüm hesaplar durduruldu'); onAccountsChange() }
      else toast.error(res.error || 'Durdurma hatası')
    } catch { toast.error('Hata oluştu') }
  }

  const handleLaunchSelected = async () => {
    if (!selected.length) { toast.error('Önce hesap seçin'); return }



    for (let i = 0; i < selected.length; i++) {
      const id = selected[i];
      await handleLaunchGame(id);

      const delay = 9000;
      await new Promise(r => setTimeout(r, delay));
    }
  }

  let displayedAccounts = showFavoritesOnly
    ? accounts.filter(a => favorites.includes(a.id))
    : [...accounts]

  if (serverFilter !== 'all') {
    displayedAccounts = displayedAccounts.filter(a => a.server_port === parseInt(serverFilter))
  }

  displayedAccounts.sort((a, b) => {
    const idxA = customOrder.indexOf(a.id)
    const idxB = customOrder.indexOf(b.id)
    if (idxA !== -1 && idxB !== -1) return idxA - idxB
    if (idxA !== -1) return -1
    if (idxB !== -1) return 1
    return 0
  })

  const saveOrder = (newOrder) => {
    setCustomOrder(newOrder)
    localStorage.setItem('ll_order', JSON.stringify(newOrder))
  }

  const formatDate = (dateStr) => {
    if (!dateStr) return 'Süresiz';
    return new Date(dateStr).toLocaleString('tr-TR', {
      day: 'numeric', month: 'long', year: 'numeric', hour: '2-digit', minute: '2-digit'
    });
  }

  // ── Pointer-based drag & drop (replaces broken HTML5 DnD) ──
  const dragRef = useRef({ sourceIndex: null, active: false })

  const handleGripDown = useCallback((e, index) => {
    e.preventDefault()
    e.stopPropagation()
    dragRef.current = { sourceIndex: index, active: true }
    setDragState({ dragging: index, over: null })

    const handlePointerMove = (ev) => {
      const el = document.elementFromPoint(ev.clientX, ev.clientY)
      if (el) {
        const card = el.closest('[data-card-index]')
        if (card) {
          const overIdx = parseInt(card.dataset.cardIndex, 10)
          setDragState(prev => ({ ...prev, over: overIdx }))
        }
      }
    }

    const handlePointerUp = () => {
      document.removeEventListener('pointermove', handlePointerMove)
      document.removeEventListener('pointerup', handlePointerUp)

      const src = dragRef.current.sourceIndex
      setDragState(prev => {
        const tgt = prev.over
        if (src !== null && tgt !== null && src !== tgt) {
          const ids = displayedAccounts.map(a => a.id)
          const dragged = ids.splice(src, 1)[0]
          ids.splice(tgt, 0, dragged)
          saveOrder(ids)
        }
        return { dragging: null, over: null }
      })
      dragRef.current = { sourceIndex: null, active: false }
    }

    document.addEventListener('pointermove', handlePointerMove)
    document.addEventListener('pointerup', handlePointerUp)
  }, [displayedAccounts, saveOrder])

  const navItems = [
    { id: 'accounts', icon: <IconRun />, label: 'Hesaplar' },
    { id: 'settings', icon: <IconSettings />, label: 'Ayarlar' },
  ]

  return (
    <div className="app-layout">
      {/* ── Sidebar ── */}
      <aside className="sidebar">
        <div style={{ marginBottom: 16 }}>
          <img src={logoImg} alt="Logo" style={{ width: 42, height: 42, borderRadius: 8, objectFit: 'cover' }} />
        </div>

        <nav className="sidebar-nav">
          {navItems.map(item => {
            return (
              <button
                key={item.id}
                className={`sidebar-btn${activeTab === item.id ? ' active' : ''}`}
                onClick={() => setActiveTab(item.id)}
                title={item.label || undefined}
              >{item.icon}</button>
            )
          })}
        </nav>

        <div className="sidebar-bottom">
          <button className="sidebar-btn" onClick={handleLogout} title="Yenile"><IconRefresh /></button>
        </div>
      </aside>

      {/* ── Main ── */}
      <div className="main-content">

        {activeTab === 'accounts' && (
          <div style={{ flex: 1, display: 'flex', flexDirection: 'column', position: 'relative', overflow: 'hidden' }}>
              <>
                {/* Topbar */}
                <div className="topbar">
                  {/* Logo text */}
                  <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginRight: 16 }}>
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                      <span style={{
                        fontFamily: "'Space Grotesk', sans-serif",
                        fontWeight: 800,
                        fontSize: 18,
                        color: '#e8eaf0',
                        letterSpacing: '-0.5px',
                        lineHeight: 1
                      }}>
                        <img src={logoImg} alt="Logo" style={{ width: 42, height: 42, borderRadius: 8, objectFit: 'cover' }} />
                      </span>
                    </div>
                  </div>

                  <button className="btn-add" onClick={() => {
                    setNewForm({ nickname: '', username: '', password: '', server: '93.155.105.236', server_port: 27201 });
                    setEditingAccountId(null);
                    setShowModal(true);
                  }} title="Hesap Ekle">+</button>

                  <button
                    className="btn-favorites"
                    onClick={() => setShowFavoritesOnly(v => !v)}
                    style={showFavoritesOnly ? { background: 'linear-gradient(135deg,#d97706,#b45309)' } : {}}
                  >
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" /></svg>
                    FAVORİLER
                  </button>

                  <button className="btn-start-selected" onClick={handleLaunchSelected} disabled={!selected.length}>
                    <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3" /></svg>
                    SEÇİLİYİ BAŞLAT
                  </button>

                  <button className="btn-start-all" onClick={handleLaunchAll} disabled={launchAllLoading || !accounts.length}>
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                      <polygon points="5 3 19 12 5 21 5 3" /><line x1="19" y1="3" x2="19" y2="21" />
                    </svg>
                    TÜMÜNÜ BAŞLAT
                  </button>

                  <button className="btn-start-all" onClick={handleStopAll} disabled={!accounts.length} style={{ marginLeft: 8, background: 'rgba(239,68,68,0.15)', color: 'var(--red)', border: '1px solid rgba(239,68,68,0.3)' }}>
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="5" width="14" height="14" /></svg>
                    TÜMÜNÜ DURDUR
                  </button>

                  <span className="topbar-spacer" />

                  <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                    <select
                      value={serverFilter}
                      onChange={(e) => setServerFilter(e.target.value)}
                      style={{
                        background: 'rgba(255,255,255,0.05)',
                        border: '1px solid var(--border)',
                        color: 'var(--text-secondary)',
                        padding: '6px 12px',
                        borderRadius: '12px',
                        fontSize: '13px',
                        cursor: 'pointer',
                        outline: 'none'
                      }}
                    >
                      <option value="all" style={{ background: '#1e1e24', color: '#fff' }}>Tüm Sunucular</option>
                      {SERVERS.map(s => (
                        <option key={s.label} value={s.port} style={{ background: '#1e1e24', color: '#fff' }}>
                          {s.label}
                        </option>
                      ))}
                    </select>

                    <button className="btn-icon-sm" title={hideInfo ? "Bilgileri Göster" : "Bilgileri Gizle"} onClick={() => setHideInfo(!hideInfo)}>
                      {hideInfo ? (
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24M1 1l22 22" /></svg>
                      ) : (
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /><circle cx="12" cy="12" r="3" /></svg>
                      )}
                    </button>

                    <button className="btn-icon-sm" title="Profil" onClick={() => setShowProfilePopup(true)}>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" /></svg>
                    </button>
                  </div>
                </div>

                {/* Grid */}
                <div className="accounts-container">
                  {loading ? (
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: 200, color: 'var(--text-muted)', gap: 10 }}>
                      <svg className="spinner" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                      </svg>
                      Hesaplar yükleniyor...
                    </div>
                  ) : displayedAccounts.length === 0 ? (
                    <div className="empty-state">
                      <svg className="empty-state-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                        <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
                        <circle cx="9" cy="7" r="4" />
                        <line x1="19" y1="8" x2="19" y2="14" /><line x1="22" y1="11" x2="16" y2="11" />
                      </svg>
                      <p style={{ fontSize: 14, fontWeight: 600, color: 'var(--text-secondary)' }}>
                        {showFavoritesOnly ? 'Favori hesap yok' : 'Henüz hesap eklenmemiş'}
                      </p>
                      <p style={{ fontSize: 12, color: 'var(--text-muted)' }}>
                        {showFavoritesOnly ? 'Yıldız ikonuna tıklayarak favorilere ekleyin' : '+ butonuna tıklayarak hesap ekleyin'}
                      </p>
                    </div>
                  ) : (
                    <div className="accounts-grid">
                      {displayedAccounts.map((account, index) => (
                        <div
                          key={account.id}
                          data-card-index={index}
                          className={dragState.over === index && dragState.dragging !== index ? 'drag-over' : ''}
                          style={{
                            opacity: dragState.dragging === index ? 0.4 : 1,
                            transform: dragState.dragging === index ? 'scale(0.95)' : 'scale(1)',
                            transition: 'all 0.15s ease',
                            borderRadius: 'var(--radius-lg)',
                            position: 'relative',
                            zIndex: openPopupAccountId === account.id ? 100 : 1
                          }}
                        >
                          {/* Drag Handle */}
                          <div
                            className="drag-handle"
                            onPointerDown={(e) => handleGripDown(e, index)}
                            title="Sürükle"
                          >
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                              <circle cx="9" cy="5" r="1.5" /><circle cx="15" cy="5" r="1.5" />
                              <circle cx="9" cy="12" r="1.5" /><circle cx="15" cy="12" r="1.5" />
                              <circle cx="9" cy="19" r="1.5" /><circle cx="15" cy="19" r="1.5" />
                            </svg>
                          </div>
                          <AccountCard
                            account={account}
                            index={index}
                            nickname={nicknames[account.id]}
                            windowSize={windowSizes[account.id] || 'small'}
                            lastCustomSize={customSizes[account.id]}
                            hideInfo={hideInfo}
                            isFavorite={favorites.includes(account.id)}
                            isSelected={selected.includes(account.id)}
                            onToggleFavorite={() => toggleFavorite(account.id)}
                            onToggleSelect={() => toggleSelect(account.id)}
                            onChangeWindowSize={(size) => changeWindowSize(account.id, size)}
                            onLaunch={() => handleLaunchGame(account.id)}
                            onRemove={() => handleRemoveAccount(account.id)}
                            onStop={() => handleStopAccount(account.id)}
                            onEdit={() => {
                              setNewForm({
                                nickname: nicknames[account.id] || '',
                                username: account.username,
                                password: account.password,
                                server: account.server,
                                server_port: account.server_port
                              });
                              setEditingAccountId(account.id);
                              setShowModal(true);
                            }}
                            onPopupToggle={(isOpen) => setOpenPopupAccountId(isOpen ? account.id : null)}
                          />
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </>
          </div>
        )}



        {activeTab === 'settings' && (
          <div style={{ flex: 1, overflow: 'auto', padding: 20 }}>
            <SettingsPanel />
          </div>
        )}

        {/* Status Bar */}
        <div className="status-bar">
          <div className="running-dot" style={{ width: 10, height: 10, boxShadow: '0 0 10px var(--green)' }} />
          LimitLess - İstanbul Kıyamet Vakti Yöneticisi
        </div>
      </div>

      {/* ── Add Account Modal ── */}
      {showModal && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <div className="modal-title">
                <span className="modal-title-icon">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
                    <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
                    <circle cx="9" cy="7" r="4" />
                    <line x1="19" y1="8" x2="19" y2="14" /><line x1="22" y1="11" x2="16" y2="11" />
                  </svg>
                </span>
                {editingAccountId ? 'Hesabı Düzenle' : 'Yeni Hesap Ekle'}
              </div>
              <button className="modal-close" onClick={() => setShowModal(false)}>✕</button>
            </div>

            <div className="modal-body">
              <div className="modal-grid">
                <div className="modal-field">
                  <label className="modal-label">Karakter Adı</label>
                  <input
                    className="modal-input"
                    placeholder="Karakter Adı"
                    value={newForm.nickname}
                    onChange={e => setNewForm({ ...newForm, nickname: e.target.value })}
                  />
                </div>
                <div className="modal-field">
                  <label className="modal-label">Kullanıcı Adı</label>
                  <input
                    className="modal-input"
                    placeholder="Telefon veya Mail"
                    value={newForm.username}
                    onChange={e => setNewForm({ ...newForm, username: e.target.value })}
                  />
                </div>
                <div className="modal-field">
                  <label className="modal-label">Şifre</label>
                  <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                    <input
                      className="modal-input"
                      type={showPassword ? "text" : "password"}
                      placeholder="••••••••"
                      value={newForm.password}
                      onChange={e => setNewForm({ ...newForm, password: e.target.value })}
                      onKeyDown={e => e.key === 'Enter' && handleSaveAccount()}
                      style={{ paddingRight: '40px', width: '100%' }}
                    />
                    <button
                      type="button"
                      onClick={() => setShowPassword(!showPassword)}
                      style={{
                        position: 'absolute',
                        right: '10px',
                        background: 'transparent',
                        border: 'none',
                        color: 'var(--text-muted)',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        padding: '4px'
                      }}
                      title={showPassword ? "Şifreyi Gizle" : "Şifreyi Göster"}
                    >
                      {showPassword ? (
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
                          <line x1="1" y1="1" x2="23" y2="23"></line>
                        </svg>
                      ) : (
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                          <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                          <circle cx="12" cy="12" r="3"></circle>
                        </svg>
                      )}
                    </button>
                  </div>
                </div>
                <div className="modal-field">
                  <label className="modal-label">Sunucu</label>
                  <select
                    className="modal-select"
                    value={`${newForm.server}:${newForm.server_port}`}
                    onChange={e => {
                      const srv = SERVERS.find(s => `${s.value}:${s.port}` === e.target.value)
                      if (srv) setNewForm({ ...newForm, server: srv.value, server_port: srv.port })
                    }}
                  >
                    {SERVERS.map(s => (
                      <option key={s.label} value={`${s.value}:${s.port}`}>{s.label}</option>
                    ))}
                  </select>
                </div>
              </div>

              <button className="modal-submit" onClick={handleSaveAccount} disabled={addLoading}>
                {addLoading ? (
                  <>
                    <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                      <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                    </svg>
                    {editingAccountId ? 'Güncelleniyor...' : 'Ekleniyor...'}
                  </>
                ) : (
                  <>
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                      <line x1="12" y1="5" x2="12" y2="19" /><line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                    {editingAccountId ? 'KAYDET' : 'HESABI EKLE'}
                  </>
                )}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* ── Profile Modal ── */}
      {showProfilePopup && (
        <div className="modal-overlay" onClick={() => setShowProfilePopup(false)}>
          <div className="modal" onClick={e => e.stopPropagation()} style={{ maxWidth: '350px' }}>
            <div className="modal-header">
              <div className="modal-title">
                <span className="modal-title-icon">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" />
                  </svg>
                </span>
                Kullanıcı Profili
              </div>
              <button className="modal-close" onClick={() => setShowProfilePopup(false)}>✕</button>
            </div>
            <div className="modal-body" style={{ padding: '24px 20px' }}>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                  <div style={{ textAlign: 'center', marginBottom: 8 }}>
                    <div style={{
                      width: 64, height: 64, borderRadius: '50%', background: 'var(--accent)',
                      display: 'flex', alignItems: 'center', justifyContent: 'center',
                      margin: '0 auto 12px', color: '#fff', fontSize: 24, fontWeight: 'bold'
                    }}>
                      Y
                    </div>
                    <div style={{ fontSize: 18, fontWeight: 700, color: '#fff' }}>Yerel Kullanıcı</div>
                    <div style={{ fontSize: 12, color: 'var(--text-muted)', marginTop: 4 }}>Aktif (Çevrimdışı)</div>
                  </div>

                  <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    <div style={{
                      background: 'rgba(255,255,255,0.03)', border: '1px solid var(--border)',
                      borderRadius: 12, padding: 12, display: 'flex', justifyContent: 'space-between', alignItems: 'center'
                    }}>
                      <span style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Hesap Durumu</span>
                      <span style={{ fontSize: 13, color: '#10b981', fontWeight: 600 }}>Sınırsız ✓</span>
                    </div>

                    <div style={{
                      background: 'rgba(255,255,255,0.03)', border: '1px solid var(--border)',
                      borderRadius: 12, padding: 12, display: 'flex', justifyContent: 'space-between', alignItems: 'center'
                    }}>
                      <span style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Kullanıcı Yetkisi</span>
                      <span style={{ fontSize: 13, color: '#ef4444', fontWeight: 600 }}>
                        Yerel Yönetici
                      </span>
                    </div>

                    <div style={{
                      background: 'rgba(255,255,255,0.03)', border: '1px solid var(--border)',
                      borderRadius: 12, padding: 12, display: 'flex', justifyContent: 'space-between', alignItems: 'center'
                    }}>
                      <span style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Erişim Paketleri</span>
                      <span style={{ fontSize: 13, color: '#8b5cf6', fontWeight: 600 }}>
                        Hızlı Giriş
                      </span>
                    </div>

                    <div style={{
                      background: 'rgba(255,255,255,0.03)', border: '1px solid var(--border)',
                      borderRadius: 12, padding: 12, display: 'flex', justifyContent: 'space-between', alignItems: 'center'
                    }}>
                      <span style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Uygulama Sürümü</span>
                      <span style={{ fontSize: 13, color: 'var(--text-primary)', fontWeight: 600 }}>v{appVersion || '1.1.7'}</span>
                    </div>

                    <div style={{
                      background: 'rgba(255,255,255,0.03)', border: '1px solid var(--border)',
                      borderRadius: 12, padding: 12, display: 'flex', justifyContent: 'space-between', alignItems: 'center'
                    }}>
                      <span style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Kayıtlı Hesaplar</span>
                      <span style={{ fontSize: 13, color: 'var(--text-primary)', fontWeight: 600 }}>{accounts.length} Adet</span>
                    </div>
                  </div>
                </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
// ─── Account Card ─────────────────────────────────────────────────────────────
function AccountCard({ account, index, nickname, windowSize, lastCustomSize, hideInfo, isFavorite, isSelected, onToggleFavorite, onToggleSelect, onChangeWindowSize, onLaunch, onRemove, onStop, onEdit, onPopupToggle }) {
  const [launching, setLaunching] = useState(false)
  const [removing, setRemoving] = useState(false)
  const [stopping, setStopping] = useState(false)
  const [showCustomSize, setShowCustomSize] = useState(false)
  const [customWidth, setCustomWidth] = useState('')
  const [customHeight, setCustomHeight] = useState('')
  const [popupDirection, setPopupDirection] = useState('down')

  const handleLaunch = async () => { setLaunching(true); await onLaunch(); setLaunching(false) }
  const handleRemove = async () => { setRemoving(true); await onRemove(); setRemoving(false) }
  const handleStop = async () => { setStopping(true); await onStop(); setStopping(false) }

  // Parse existing custom size for display
  useEffect(() => {
    if (windowSize && windowSize.startsWith('custom_')) {
      const parts = windowSize.replace('custom_', '').split('x')
      if (parts.length === 2) {
        setCustomWidth(parts[0])
        setCustomHeight(parts[1])
      }
    }
  }, [])

  const handleWindowSizeChange = (e) => {
    const val = e.target.value;
    if (val === 'custom') {
      if (lastCustomSize) {
        onChangeWindowSize(lastCustomSize);
      } else {
        const rect = e.target.getBoundingClientRect();
        setPopupDirection(rect.top > window.innerHeight / 2 ? 'up' : 'down');
        setCustomWidth('1024')
        setCustomHeight('768')
        setShowCustomSize(true)
        if (onPopupToggle) onPopupToggle(true)
      }
    } else if (val === 'custom_trigger') {
      const rect = e.target.getBoundingClientRect();
      setPopupDirection(rect.top > window.innerHeight / 2 ? 'up' : 'down');
      // Load current or last custom size
      let sourceSize = (windowSize && windowSize.startsWith('custom_')) ? windowSize : (lastCustomSize || '');
      if (sourceSize.startsWith('custom_')) {
        const parts = sourceSize.replace('custom_', '').split('x')
        if (parts.length === 2) {
          setCustomWidth(parts[0])
          setCustomHeight(parts[1])
        }
      } else {
        setCustomWidth('1024')
        setCustomHeight('768')
      }
      setShowCustomSize(true)
      if (onPopupToggle) onPopupToggle(true)
    } else {
      onChangeWindowSize(val)
    }
  }

  const handleSaveCustomSize = () => {
    const w = parseInt(customWidth) || 800
    const h = parseInt(customHeight) || 600
    onChangeWindowSize(`custom_${w}x${h}`)
    setShowCustomSize(false)
    if (onPopupToggle) onPopupToggle(false)
  }

  const baseName = nickname || account.username;
  const displayName = hideInfo ? 'Gizli Karakter' : baseName;
  const displayId = hideInfo ? '***' : account.username;
  const serverName = SERVERS.find(s => s.port === account.server_port)?.label || 'Bilinmeyen'

  // Display value for select
  const selectValue = windowSize && windowSize.startsWith('custom_') ? 'custom' : windowSize

  return (
    <div
      className={`account-card animate-fade-in${isSelected ? ' selected' : ''}`}
      style={{ animationDelay: `${index * 0.04}s`, zIndex: showCustomSize ? 100 : 1 }}
      onClick={onToggleSelect}
    >
      <div className="card-top">
        <div className="card-info">
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
            <div
              className="running-dot"
              style={{
                width: '10px',
                height: '10px',
                minWidth: '10px',
                flexShrink: 0,
                borderRadius: '50%',
                background: account.is_running ? '#22c55e' : '#ef4444',
                boxShadow: account.is_running ? '0 0 10px #22c55e' : '0 0 10px #ef4444'
              }}
              title={account.is_running ? "Hesap Açık" : "Hesap Kapalı"}
            />
            <div className="card-name" title={displayName} style={{ marginBottom: 0 }}>
              {displayName.length > 20 ? displayName.slice(0, 20) + '…' : displayName}
            </div>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: 8 }}>
            <div className="card-id">
              <span className="card-id-dot" style={{ background: '#38bdf8' }} />
              <span style={{ color: '#38bdf8', marginRight: 4, fontWeight: 700 }}>{serverName}</span>
              <span style={{ color: 'var(--border)' }}>|</span>
              <span style={{ marginLeft: 4 }}>ID: {displayId.length > 15 ? displayId.slice(0, 15) + '…' : displayId}</span>
            </div>
          </div>
        </div>
        <button
          className={`star-btn${isFavorite ? ' active' : ''}`}
          onClick={e => { e.stopPropagation(); onToggleFavorite() }}
          title={isFavorite ? 'Favoriden kaldır' : 'Favorilere ekle'}
          style={{ position: 'absolute', right: 12, top: 12 }}
        >
          <IconStar filled={isFavorite} />
        </button>
      </div>

      <div className="card-actions" onClick={e => e.stopPropagation()}>
        {/* Delete */}
        <button className="card-btn danger" onClick={handleRemove} disabled={removing} title="Sil">
          {removing
            ? <svg className="spinner" width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4" /></svg>
            : <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2-2v2" /></svg>
          }
        </button>

        {/* Edit */}
        <button className="card-btn" title="Düzenle" onClick={e => { e.stopPropagation(); onEdit(); }}>
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" /><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" /></svg>
        </button>

        {/* Stop */}
        <button className="card-btn stop" onClick={e => { e.stopPropagation(); handleStop() }} disabled={stopping} title="Durdur">
          {stopping ? (
            <svg className="spinner" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" /></svg>
          ) : (
            <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="5" width="14" height="14" rx="2" /></svg>
          )}
        </button>



        <span className="card-actions-spacer" />

        {/* Window Size */}
        <div style={{ position: 'relative', marginRight: 6 }}>
          <select
            className="card-btn"
            value={selectValue}
            onChange={e => { e.stopPropagation(); handleWindowSizeChange(e); }}
            onClick={e => e.stopPropagation()}
            title="Pencere Boyutu"
            style={{
              appearance: 'none',
              background: 'rgba(255,255,255,0.05)',
              border: '1px solid rgba(255,255,255,0.1)',
              color: 'var(--text-secondary)',
              padding: '4px 14px 4px 4px',
              borderRadius: '6px',
              fontSize: '11px',
              cursor: 'pointer',
              outline: 'none',
              height: '100%',
              display: 'flex',
              alignItems: 'center',
              width: '80px',
              textOverflow: 'ellipsis'
            }}
          >
            <option value="small" style={{ background: '#1e1e24', color: '#fff' }}>600p</option>
            <option value="medium" style={{ background: '#1e1e24', color: '#fff' }}>720p</option>
            <option value="large" style={{ background: '#1e1e24', color: '#fff' }}>1080p</option>
            <option value="2k_4" style={{ background: '#1e1e24', color: '#fff' }}>2K (4)</option>
            <option value="2k_2" style={{ background: '#1e1e24', color: '#fff' }}>2K (2)</option>
            <option value="2k_full" style={{ background: '#1e1e24', color: '#fff' }}>2K (Tam)</option>
            {selectValue === 'custom' ? (
              <>
                <option value="custom" style={{ background: '#1e1e24', color: '#fff' }}>Özel</option>
                <option value="custom_trigger" style={{ background: '#1e1e24', color: '#fb923c' }}>✏️ Özel Boyutu Düzenle</option>
              </>
            ) : (
              <option value="custom" style={{ background: '#1e1e24', color: '#fff' }}>Özel</option>
            )}
          </select>
          <div style={{ position: 'absolute', right: '4px', top: '50%', transform: 'translateY(-50%)', pointerEvents: 'none', color: 'var(--text-secondary)', display: 'flex' }}>
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M6 9l6 6 6-6" /></svg>
          </div>
        </div>

        {/* Play */}
        <button className="card-btn play" onClick={e => { e.stopPropagation(); handleLaunch() }} disabled={launching}>
          {launching ? (
            <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" /></svg>
          ) : (
            <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><polygon points="5 3 19 12 5 21 5 3" /></svg>
          )}
          OYNA
        </button>
      </div>

      {/* Custom Size Popup */}
      {showCustomSize && (
        <div
          onClick={e => e.stopPropagation()}
          style={{
            position: 'absolute',
            top: popupDirection === 'down' ? '100%' : 'auto',
            bottom: popupDirection === 'up' ? '100%' : 'auto',
            marginTop: popupDirection === 'down' ? '8px' : '0',
            marginBottom: popupDirection === 'up' ? '8px' : '0',
            right: 0,
            background: 'linear-gradient(135deg, rgba(30, 35, 45, 0.95), rgba(20, 25, 35, 0.95))',
            backdropFilter: 'blur(30px)',
            border: '1px solid rgba(255,255,255,0.15)',
            borderRadius: '14px',
            padding: '16px',
            boxShadow: '0 20px 50px rgba(0,0,0,0.6)',
            zIndex: 100,
            width: '220px',
            animation: 'fadeIn 0.15s ease'
          }}
        >
          <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-primary)', marginBottom: '12px', fontFamily: 'Space Grotesk, sans-serif', letterSpacing: '0.05em' }}>
            ÖZEL BOYUT
          </div>
          <div style={{ display: 'flex', gap: '8px', marginBottom: '12px' }}>
            <div style={{ flex: 1 }}>
              <label style={{ fontSize: '10px', color: 'var(--text-muted)', fontWeight: 600, letterSpacing: '0.06em', display: 'block', marginBottom: '4px' }}>EN (px)</label>
              <input
                type="number"
                value={customWidth}
                onChange={e => setCustomWidth(e.target.value)}
                placeholder="1024"
                style={{
                  width: '100%',
                  background: 'rgba(255,255,255,0.06)',
                  border: '1px solid rgba(255,255,255,0.1)',
                  borderRadius: '8px',
                  color: '#fff',
                  padding: '8px 10px',
                  fontSize: '13px',
                  fontFamily: 'Inter, sans-serif',
                  outline: 'none',
                  textAlign: 'center',
                }}
                onFocus={e => { e.target.style.borderColor = 'rgba(194,122,81,0.6)'; e.target.style.boxShadow = '0 0 0 3px rgba(194,122,81,0.15)'; }}
                onBlur={e => { e.target.style.borderColor = 'rgba(255,255,255,0.1)'; e.target.style.boxShadow = 'none'; }}
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'flex-end', paddingBottom: '8px', color: 'var(--text-muted)', fontWeight: 700, fontSize: '14px' }}>×</div>
            <div style={{ flex: 1 }}>
              <label style={{ fontSize: '10px', color: 'var(--text-muted)', fontWeight: 600, letterSpacing: '0.06em', display: 'block', marginBottom: '4px' }}>BOY (px)</label>
              <input
                type="number"
                value={customHeight}
                onChange={e => setCustomHeight(e.target.value)}
                placeholder="768"
                style={{
                  width: '100%',
                  background: 'rgba(255,255,255,0.06)',
                  border: '1px solid rgba(255,255,255,0.1)',
                  borderRadius: '8px',
                  color: '#fff',
                  padding: '8px 10px',
                  fontSize: '13px',
                  fontFamily: 'Inter, sans-serif',
                  outline: 'none',
                  textAlign: 'center',
                }}
                onFocus={e => { e.target.style.borderColor = 'rgba(194,122,81,0.6)'; e.target.style.boxShadow = '0 0 0 3px rgba(194,122,81,0.15)'; }}
                onBlur={e => { e.target.style.borderColor = 'rgba(255,255,255,0.1)'; e.target.style.boxShadow = 'none'; }}
              />
            </div>
          </div>
          <div style={{ display: 'flex', gap: '6px' }}>
            <button
              onClick={() => {
                setShowCustomSize(false);
                if (onPopupToggle) onPopupToggle(false);
              }}
              style={{
                flex: 1,
                padding: '8px',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.08)',
                background: 'rgba(255,255,255,0.05)',
                color: 'var(--text-secondary)',
                fontSize: '11px',
                fontWeight: 600,
                cursor: 'pointer',
                fontFamily: 'Inter, sans-serif',
              }}
            >
              İptal
            </button>
            <button
              onClick={handleSaveCustomSize}
              style={{
                flex: 1,
                padding: '8px',
                borderRadius: '8px',
                border: 'none',
                background: 'linear-gradient(135deg, var(--accent), var(--accent-hover))',
                color: '#fff',
                fontSize: '11px',
                fontWeight: 700,
                cursor: 'pointer',
                fontFamily: 'Inter, sans-serif',
                boxShadow: '0 2px 10px var(--accent-glow)',
              }}
            >
              Kaydet
            </button>
          </div>
          {customWidth && customHeight && (
            <div style={{ marginTop: '8px', textAlign: 'center', fontSize: '10px', color: 'var(--text-muted)' }}>
              Önizleme: {customWidth} × {customHeight}
            </div>
          )}
        </div>
      )}
    </div>
  )
}

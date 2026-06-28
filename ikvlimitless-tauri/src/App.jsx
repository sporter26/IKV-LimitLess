import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { toast, modalEventEmitter } from './utils/modal'
import Dashboard from './pages/Dashboard'
import AnimatedBackground from './components/AnimatedBackground'
import { applyTheme, getSavedThemeId } from './utils/theme'

import './App.css'



function App() {


  const [accounts, setAccounts] = useState([])
  const [loading, setLoading] = useState(true)
  const [toasts, setToasts] = useState([])

  useEffect(() => {
    const handleModal = (e) => {
      const newToast = { id: Date.now() + Math.random(), message: e.detail.message, type: e.detail.type };
      setToasts(prev => [...prev, newToast]);
      setTimeout(() => {
        setToasts(prev => prev.filter(t => t.id !== newToast.id));
      }, 4000);
    };
    modalEventEmitter.addEventListener('showModal', handleModal);
    return () => modalEventEmitter.removeEventListener('showModal', handleModal);
  }, []);

  useEffect(() => {
    applyTheme(getSavedThemeId())
    loadAccounts();
  }, [])

  const loadAccounts = async () => {
    try {
      const response = await invoke('get_accounts')
      if (response.success && response.data) {
        const nicknamesMap = {}
        try { Object.assign(nicknamesMap, JSON.parse(localStorage.getItem('ll_nicknames') || '{}')) } catch {}
        const accountsWithNicknames = response.data.map(acc => ({ ...acc, nickname: nicknamesMap[acc.id] || '' }))
        setAccounts(accountsWithNicknames)
      }
    } catch (error) {
      console.error('Hesapları yüklerken hata:', error)
      toast.error('Hesaplar yüklenemedi')
    } finally {
      setLoading(false)
    }
  }



  return (
    <>
      <AnimatedBackground />
      {/* TOAST NOTIFICATIONS */}
      <div style={{ position: 'fixed', bottom: '24px', right: '24px', zIndex: 9999, display: 'flex', flexDirection: 'column', gap: '12px' }}>
        {toasts.map(t => (
          <div key={t.id} className="animate-fade-in" style={{
            background: 'rgba(10, 15, 25, 0.95)', border: '1px solid rgba(255,255,255,0.1)', 
            borderLeft: `4px solid ${t.type === 'error' ? '#ef4444' : t.type === 'success' ? '#22c55e' : '#3b82f6'}`,
            borderRadius: '10px', padding: '16px 20px', minWidth: '280px', maxWidth: '350px',
            backdropFilter: 'blur(10px)', boxShadow: '0 10px 30px rgba(0,0,0,0.5)', 
            display: 'flex', alignItems: 'flex-start', gap: '12px'
          }}>
             <div style={{ color: t.type === 'error' ? '#ef4444' : t.type === 'success' ? '#22c55e' : '#3b82f6', marginTop: '2px' }}>
                {t.type === 'error' && <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>}
                {t.type === 'success' && <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>}
                {t.type === 'info' && <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>}
             </div>
             <div style={{ flex: 1 }}>
               <div style={{ color: '#fff', fontSize: '14px', fontWeight: 'bold', marginBottom: '4px' }}>
                 {t.type === 'error' ? 'Hata' : t.type === 'success' ? 'Başarılı' : 'Bilgi'}
               </div>
               <div style={{ color: '#cbd5e1', fontSize: '13px', lineHeight: '1.4' }}>{t.message}</div>
             </div>
             <button onClick={() => setToasts(prev => prev.filter(x => x.id !== t.id))} style={{ background: 'transparent', border: 'none', color: '#64748b', cursor: 'pointer', padding: '0', marginLeft: '8px' }}>
               <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
             </button>
          </div>
        ))}
      </div>
      <Dashboard
        accounts={accounts}
        onAccountsChange={loadAccounts}
        loading={loading}
      />
    </>
  )
}

export default App

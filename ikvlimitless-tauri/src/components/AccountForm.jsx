export default function AccountForm({ form, setForm, onSubmit, loading }) {
  return (
    <div className="glass-card">
      <div className="section-header">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" strokeWidth="2">
          <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
          <circle cx="9" cy="7" r="4" />
          <line x1="19" y1="8" x2="19" y2="14" />
          <line x1="22" y1="11" x2="16" y2="11" />
        </svg>
        <span className="section-title">Yeni Hesap Ekle</span>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px', marginBottom: '12px' }}>
        <div>
          <label className="field-label">GSM / E-Posta</label>
          <input
            type="text"
            className="input-field"
            placeholder="05xx... veya abc@..."
            value={form.username}
            onChange={(e) => setForm({ ...form, username: e.target.value })}
            onKeyDown={(e) => e.key === 'Enter' && onSubmit()}
          />
        </div>

        <div>
          <label className="field-label">Şifre</label>
          <input
            type="password"
            className="input-field"
            placeholder="••••••••"
            value={form.password}
            onChange={(e) => setForm({ ...form, password: e.target.value })}
            onKeyDown={(e) => e.key === 'Enter' && onSubmit()}
          />
        </div>

        <div>
          <label className="field-label">Sunucu</label>
          <select
            className="input-field"
            value={form.server}
            onChange={(e) => setForm({ ...form, server: e.target.value })}
            style={{ cursor: 'pointer' }}
          >
            <option value="94.103.32.22">Varsayılan Sunucu (94.103.32.22)</option>
            <option value="93.155.105.236">Alternatif Sunucu (93.155.105.236)</option>
            <option value="127.0.0.1">Yerel Sunucu (Localhost)</option>
          </select>
        </div>

        <div>
          <label className="field-label">Port</label>
          <input
            type="number"
            className="input-field"
            placeholder="27206"
            value={form.server_port}
            onChange={(e) => setForm({ ...form, server_port: parseInt(e.target.value) || 0 })}
          />
        </div>
      </div>

      <button
        className="btn btn-primary"
        onClick={onSubmit}
        disabled={loading}
        style={{ width: '100%' }}
      >
        {loading ? (
          <>
            <svg className="spinner" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
            </svg>
            Ekleniyor...
          </>
        ) : (
          <>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <line x1="12" y1="5" x2="12" y2="19" />
              <line x1="5" y1="12" x2="19" y2="12" />
            </svg>
            Hesap Ekle
          </>
        )}
      </button>
    </div>
  )
}

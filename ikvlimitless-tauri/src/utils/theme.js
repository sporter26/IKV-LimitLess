export const THEMES = [
  { id: 'orange', name: 'Turuncu (Varsayılan)', accent: '#c27a51', hover: '#a3613d', glow: 'rgba(194,122,81,0.25)' },
  { id: 'cyan', name: 'Mavi', accent: '#00f0ff', hover: '#00c3cc', glow: 'rgba(0,240,255,0.25)' },
  { id: 'green', name: 'Yeşil', accent: '#10b981', hover: '#059669', glow: 'rgba(16,185,129,0.25)' },
  { id: 'purple', name: 'Mor', accent: '#b584ff', hover: '#9333ea', glow: 'rgba(181,132,255,0.25)' },
  { id: 'red', name: 'Kırmızı', accent: '#ef4444', hover: '#dc2626', glow: 'rgba(239,68,68,0.25)' }
]

export const applyTheme = (themeId) => {
  const theme = THEMES.find(t => t.id === themeId) || THEMES[0];
  document.documentElement.style.setProperty('--accent', theme.accent);
  document.documentElement.style.setProperty('--accent-hover', theme.hover);
  document.documentElement.style.setProperty('--accent-glow', theme.glow);
  localStorage.setItem('ll_theme', theme.id);
  window.dispatchEvent(new CustomEvent('theme_changed', { detail: theme }));
}

export const getSavedThemeId = () => {
  return localStorage.getItem('ll_theme') || 'orange';
}

export const BG_THEMES = [
  { id: 'network', name: 'Ağ Bağlantıları (Klasik)' },
  { id: 'orbit', name: 'Kozmik Yörünge (Büyüleyici)' },
  { id: 'fireflies', name: 'Ateş Böcekleri (Sakin)' },
  { id: 'matrix', name: 'Dijital Yağmur (Matrix)' },
  { id: 'supernova', name: 'Süpernova (Patlayıcı)' }
]

export const applyBgTheme = (bgThemeId) => {
  localStorage.setItem('ll_bg_theme', bgThemeId);
  window.dispatchEvent(new CustomEvent('bg_theme_changed', { detail: bgThemeId }));
}

export const getSavedBgTheme = () => {
  return localStorage.getItem('ll_bg_theme') || 'network';
}

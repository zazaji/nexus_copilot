// frontend/src/utils/theme.ts

/**
 * Applies the theme to the root <html> element by adding or removing the 'dark' class.
 * This is the single source of truth for theme application.
 * @param theme The theme to apply: 'light', 'dark', or 'system'.
 */
export function applyTheme(theme: 'light' | 'dark' | 'system') {
  const root = window.document.documentElement;
  
  if (theme === 'system') {
    const systemIsDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.classList.toggle('dark', systemIsDark);
  } else {
    root.classList.toggle('dark', theme === 'dark');
  }
}
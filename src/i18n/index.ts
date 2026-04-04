import { createI18n } from 'vue-i18n'
import fr from './locales/fr'
import en from './locales/en'

const savedLocale = localStorage.getItem('locale') ?? 'fr'

const i18n = createI18n({
  legacy: false,
  locale: savedLocale,
  fallbackLocale: 'fr',
  messages: { fr, en },
})

export function setLocale(locale: string) {
  ;(i18n.global.locale as any).value = locale
  localStorage.setItem('locale', locale)
}

export function getLocale(): string {
  return (i18n.global.locale as any).value
}

export default i18n

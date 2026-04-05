import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Bookmark {
  id: string
  name: string
  bucket: string
  prefix: string
}

const STORAGE_KEY = 's3-bookmarks'

export const useBookmarkStore = defineStore('bookmarks', () => {
  const bookmarks = ref<Bookmark[]>([])

  function saveToStorage() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(bookmarks.value))
  }

  function loadBookmarks() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY)
      if (raw) {
        bookmarks.value = JSON.parse(raw) as Bookmark[]
      }
    } catch {
      bookmarks.value = []
    }
  }

  function addBookmark(name: string, bucket: string, prefix: string) {
    const id = crypto.randomUUID()
    bookmarks.value.push({ id, name, bucket, prefix })
    saveToStorage()
  }

  function removeBookmark(id: string) {
    bookmarks.value = bookmarks.value.filter((b) => b.id !== id)
    saveToStorage()
  }

  // Load on store creation
  loadBookmarks()

  return {
    bookmarks,
    addBookmark,
    removeBookmark,
    loadBookmarks,
  }
})

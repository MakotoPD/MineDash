<template>
  <UApp>
    <NuxtLoadingIndicator color="aqua" errorColor="red"  />
    <div class="bg-gray-950">
      <NuxtLayout>
          <NuxtPage />
      </NuxtLayout>
    </div>
  </UApp>
</template>

<script setup lang="ts">
import { initTrayListener, cleanupTrayListener, updateTrayMenu } from '~/composables/useTray'
import { useServersStore } from '~/stores/useServersStore'
import { useSettingsStore } from '~/stores/useSettingsStore'
import { storeToRefs } from 'pinia'
import { getCurrentWindow } from '@tauri-apps/api/window'

const serversStore = useServersStore()
const { servers } = storeToRefs(serversStore)
const { loadServers, getServerStatus } = serversStore

const settingsStore = useSettingsStore()
// Ensure we have access to loaded settings
const { loadSettings } = settingsStore
const { settings } = storeToRefs(settingsStore)

let trayUpdateInterval: ReturnType<typeof setInterval> | null = null
let unlistenClose: (() => void) | null = null

onMounted(async () => {
  // Initialize tray event listener
  await initTrayListener()

  // Load Global Settings
  await loadSettings()
  
  // Setup Window Close Behavior
  const appWindow = getCurrentWindow()
  unlistenClose = await appWindow.onCloseRequested(async (event) => {
      try {
         // Check settings state directly from store
         const minimize = settings.value.minimizeOnClose
         
         if (minimize) {
            event.preventDefault()
            await appWindow.hide()
         } else {
            // Explicitly handle close to avoid ambiguity
            // We must unlisten to avoid infinite loop when we call close()
            if (unlistenClose) {
                unlistenClose()
                unlistenClose = null
            }
            await appWindow.close()
         }
      } catch (e) {
         console.error('Error handling close request', e)
         // Fallback close on error
         if (unlistenClose) {
            unlistenClose()
            unlistenClose = null
         }
         await appWindow.close()
      }
  })
  
  // Load servers for tray menu
  await loadServers()
  
  // Initial tray menu update
  await updateTrayMenu(servers.value, getServerStatus)
  
  // Periodically update tray menu with server status
  trayUpdateInterval = setInterval(async () => {
    await updateTrayMenu(servers.value, getServerStatus)
  }, 3000)
})

onUnmounted(() => {
  cleanupTrayListener()
  if (unlistenClose) unlistenClose()
  if (trayUpdateInterval) clearInterval(trayUpdateInterval)
})
</script>

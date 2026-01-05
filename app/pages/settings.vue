<template>
  <div class="min-h-screen bg-gray-950 p-8">
     <div class="max-w-2xl mx-auto space-y-8">
       <!-- Header -->
       <div class="flex items-center gap-4">
         <UButton
           icon="i-lucide-arrow-left"
           color="neutral"
           variant="ghost"
           @click="navigateTo('/')"
         />
         <div>
           <h1 class="text-3xl font-bold text-white">Settings</h1>
           <p class="text-gray-400">Configure global defaults for your servers.</p>
         </div>
       </div>

       <div class="space-y-6">
          <UCard>
             <template #header>
                <div class="flex items-center gap-2">
                   <UIcon name="i-lucide-coffee" class="text-primary-500" />
                   <h3 class="font-semibold">Java Defaults</h3>
                </div>
             </template>

             <div class="space-y-6">
                <!-- Java Status --> 
                <div class="p-4 rounded-lg border border-gray-800 bg-gray-900">
                    <div class="flex items-center justify-between mb-2">
                       <h4 class="font-medium text-sm">System Java Status</h4>
                       <UBadge :color="javaStatus.installed ? 'success' : 'warning'" variant="subtle">
                          {{ javaStatus.installed ? 'Detected' : 'Not Found' }}
                       </UBadge>
                    </div>
                    <div v-if="javaStatus.installed" class="text-xs text-gray-400">
                       {{ javaStatus.version }}
                    </div>
                    <div v-else class="text-xs text-red-500">
                       Java could not be found via 'java -version'. Please ensure it's installed and in your PATH, or set a custom path below.
                    </div>
                    <div class="mt-2">
                       <UButton size="xs" color="neutral" variant="ghost" :loading="checkingJava" @click="checkJava" icon="i-lucide-refresh-cw">
                          Check Again
                       </UButton>
                    </div>
                </div>

                <!-- Default RAM -->
                <div class="space-y-4">
                   <div class="flex justify-between">
                     <label class="text-sm font-medium">Default Memory (RAM)</label>
                     <span class="text-sm font-bold text-primary-500">{{ settings.defaultMemory }} GB</span>
                   </div>
                   <USlider v-model="settings.defaultMemory" :min="1" :max="systemRamGB" :step="0.5" />
                   <div class="flex justify-between text-xs text-gray-500">
                      <span>1 GB</span>
                      <span>{{ systemRamGB }} GB</span>
                   </div>
                </div>

                <!-- Default Flags -->
                <div class="space-y-2 flex flex-col">
                   <label class="text-sm font-medium">Global Startup Flags</label>
                   <UTextarea v-model="settings.defaultFlags" :rows="3" placeholder="-XX:+UseG1GC ..." />
                   <p class="text-xs text-gray-500">These flags are applied to all new servers by default.</p>
                </div>

                  <!-- Close Behavior -->
                  <div class="space-y-4 pt-4 border-t border-gray-800">
                     <div class="flex items-center justify-between">
                        <div>
                           <h4 class="font-medium text-sm">Window Behavior</h4>
                           <p class="text-xs text-gray-500">Configure how the application behaves when closing the window.</p>
                        </div>
                     </div>
                     
                     <div class="p-3 bg-gray-900 rounded-lg flex items-center justify-between">
                        <div class="flex items-center gap-2">
                           <UIcon name="i-lucide-minimize-2" class="w-5 h-5 text-gray-400" />
                           <span class="text-sm font-medium">Minimize to Tray on Close</span>
                        </div>
                        <USwitch v-model="settings.minimizeOnClose" color="primary" />
                     </div>
                  </div>

                  <!-- Java Installations -->
                  <div class="space-y-4 pt-4 border-t border-gray-800">
                   <div class="flex items-center justify-between">
                      <div>
                         <h4 class="font-medium text-sm">Java Installations</h4>
                         <p class="text-xs text-gray-500">Configure paths to different Java versions for compatibility</p>
                      </div>
                      <UButton size="xs" color="neutral" variant="ghost" icon="i-lucide-scan" @click="detectAllJavaVersions" :loading="detectingJava">
                         Auto-Detect
                      </UButton>
                   </div>
                   
                   <div class="space-y-3">
                      <!-- Java 8 -->
                      <div class="p-3 bg-gray-900 rounded-lg space-y-2">
                         <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                               <span class="text-sm font-medium">Java 8</span>
                               <UBadge color="neutral" variant="subtle" size="xs">MC 1.12.x and older</UBadge>
                            </div>
                            <UBadge v-if="settings.javaInstallations.java8" color="success" variant="subtle" size="xs">Configured</UBadge>
                         </div>
                         <UInput class="w-full" v-model="settings.javaInstallations.java8" placeholder="C:/Program Files/Java/jdk1.8.0_xxx/bin/java.exe" size="sm" />
                      </div>

                      <!-- Java 11 -->
                      <div class="p-3 bg-gray-900 rounded-lg space-y-2">
                         <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                               <span class="text-sm font-medium">Java 11</span>
                               <UBadge color="neutral" variant="subtle" size="xs">MC 1.13 - 1.16.x</UBadge>
                            </div>
                            <UBadge v-if="settings.javaInstallations.java11" color="success" variant="subtle" size="xs">Configured</UBadge>
                         </div>
                         <UInput class="w-full" v-model="settings.javaInstallations.java11" placeholder="C:/Program Files/Java/jdk-11/bin/java.exe" size="sm" />
                      </div>

                      <!-- Java 17 -->
                      <div class="p-3 bg-gray-900 rounded-lg space-y-2">
                         <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                               <span class="text-sm font-medium">Java 17</span>
                               <UBadge color="primary" variant="subtle" size="xs">MC 1.17 - 1.20.4 (Most Common)</UBadge>
                            </div>
                            <UBadge v-if="settings.javaInstallations.java17" color="success" variant="subtle" size="xs">Configured</UBadge>
                         </div>
                         <UInput class="w-full" v-model="settings.javaInstallations.java17" placeholder="C:/Program Files/Java/jdk-17/bin/java.exe" size="sm" />
                      </div>

                      <!-- Java 21 -->
                      <div class="p-3 bg-gray-900 rounded-lg space-y-2">
                         <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                               <span class="text-sm font-medium">Java 21</span>
                               <UBadge color="neutral" variant="subtle" size="xs">MC 1.20.5+</UBadge>
                            </div>
                            <UBadge v-if="settings.javaInstallations.java21" color="success" variant="subtle" size="xs">Configured</UBadge>
                         </div>
                         <UInput class="w-full" v-model="settings.javaInstallations.java21" placeholder="C:/Program Files/Java/jdk-21/bin/java.exe" size="sm" />
                      </div>
                   </div>
                </div>
             </div>

             <template #footer>
                <div class="flex justify-end">
                   <UButton color="primary" :loading="saving" @click="saveSettings">Save Settings</UButton>
                </div>
             </template>
          </UCard>

          <UCard>
             <template #header>
                <div class="flex items-center gap-2">
                   <UIcon name="i-lucide-info" class="text-gray-500" />
                   <h3 class="font-semibold">About MineDash</h3>
                </div>
             </template>
             <div class="space-y-4">
                <div class="flex items-center justify-between">
                   <div class="text-sm text-gray-500">
                      <p>Version: {{ currentVersion }}</p>
                      <p>Created by <a href="http://makoto.com.pl" target="_blank" rel="noopener noreferrer" class="text-primary-500 hover:underline">MakotoPD</a></p>
                   </div>
                   <UButton 
                      icon="i-lucide-refresh-cw" 
                      color="neutral" 
                      variant="ghost" 
                      :loading="checkingUpdate"
                      @click="checkUpdate"
                   >
                      Check for Updates
                   </UButton>
                </div>
                
                <!-- Update Available Banner -->
                <div v-if="updateInfo?.available" class="p-4 bg-primary-900/20 border border-primary-800 rounded-lg">
                   <div class="flex items-start gap-3">
                      <UIcon name="i-lucide-download" class="w-5 h-5 text-primary-500 mt-0.5" />
                      <div class="flex-1">
                         <h4 class="font-semibold text-primary-300">Update Available!</h4>
                         <p class="text-sm text-gray-400 mt-1">
                            Version <strong>{{ updateInfo.latestVersion }}</strong> is available. You have <strong>{{ updateInfo.currentVersion }}</strong>.
                         </p>
                         <UButton 
                            class="mt-3" 
                            size="sm" 
                            color="primary"
                            icon="i-lucide-external-link"
                            @click="openReleasePage"
                         >
                            Download Update
                         </UButton>
                      </div>
                   </div>
                </div>
                
                <!-- Up to date message -->
                <div v-else-if="updateInfo && !updateInfo.available" class="p-3 bg-success-900/20 border border-success-800 rounded-lg flex items-center gap-2">
                   <UIcon name="i-lucide-check-circle" class="w-5 h-5 text-success-500" />
                   <span class="text-sm text-success-300">You're running the latest version!</span>
                </div>
             </div>
          </UCard>
       </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-shell'
import { detectJava, type JavaStatus } from '~/utils/javaDetection'
import { useUpdateChecker } from '~/composables/useUpdateChecker'
import { GITHUB_RELEASES_URL } from '~/utils/version'
import { useSettingsStore } from '~/stores/useSettingsStore'
import { storeToRefs } from 'pinia'

const loading = ref(true)
const saving = ref(false)
const checkingJava = ref(false)
const javaStatus = ref<JavaStatus>({ installed: false, version: '', details: '' })

// Update checker
const { updateInfo, checking: checkingUpdate, checkForUpdates, currentVersion } = useUpdateChecker()

// Settings Store
const settingsStore = useSettingsStore()
const { settings, systemRamGB } = storeToRefs(settingsStore)
const { loadSettings, saveSettings } = settingsStore

const detectingJava = ref(false)

onMounted(async () => {
   loading.value = true
   await loadSettings()
   loading.value = false
   
   checkJava()
})

async function checkUpdate() {
   await checkForUpdates()
}

async function openReleasePage() {
   await open(updateInfo.value?.releaseUrl || GITHUB_RELEASES_URL)
}

async function checkJava() {
   checkingJava.value = true
   javaStatus.value = await detectJava()
   checkingJava.value = false
}

async function detectAllJavaVersions() {
   detectingJava.value = true
   try {
      // Try common Java installation paths on Windows
      const commonPaths = [
         'C:/Program Files/Java',
         'C:/Program Files/Eclipse Adoptium',
         'C:/Program Files/Zulu',
         'C:/Program Files/BellSoft',
         'C:/Program Files/Microsoft'
      ]
      
      // For now just check if default 'java' works
      const status = await detectJava()
      if (status.installed && status.version) {
         // Parse version from status
         const versionMatch = status.version.match(/(\d+)(?:\.(\d+))?/)
         if (versionMatch) {
            const major = parseInt(versionMatch[1])
            if (major === 8 || major === 1) {
               settings.value.javaInstallations.java8 = 'java'
            } else if (major >= 21) {
               settings.value.javaInstallations.java21 = 'java'
            } else if (major >= 17) {
               settings.value.javaInstallations.java17 = 'java'
            } else if (major >= 11) {
               settings.value.javaInstallations.java11 = 'java'
            }
         }
      }
   } catch (e) {
      console.error('Failed to detect Java versions', e)
   } finally {
      detectingJava.value = false
   }
}

// Wrapper for manual save button (although auto-save is active)
async function handleManualSave() {
   saving.value = true
   await saveSettings()
   saving.value = false
}

</script>

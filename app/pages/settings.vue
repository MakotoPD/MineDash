<template>
  <div class="min-h-full flex flex-col">
    <!-- Header Section -->
    <div class="relative bg-gray-900 border-b border-gray-800 pb-8 pt-8 px-8 overflow-hidden">
      <!-- Background Gradients -->
      <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-neutral-900/20 via-gray-900/0 to-transparent pointer-events-none"></div>
      <div class="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-neutral-500/20 to-transparent"></div>
      
      <div class="relative z-10">
        <div class="flex items-center gap-4 mb-2">
           <UButton
             icon="i-lucide-arrow-left"
             color="neutral"
             variant="ghost"
             to="/"
             class="hover:bg-gray-800 text-gray-400 hover:text-white transition-colors"
           />
           <div class="flex items-center gap-2 text-sm text-gray-500 font-medium uppercase tracking-wider">
              <span class="w-1.5 h-1.5 rounded-full bg-neutral-500"></span>
              Global Configuration
           </div>
        </div>
        
        <div class="ml-12">
            <h1 class="text-4xl font-bold text-white tracking-tight mb-2">Settings</h1>
            <p class="text-gray-400 text-lg max-w-2xl">Configure global defaults for your servers and manage application preferences.</p>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 p-8">
       <div class="max-w-4xl mx-auto space-y-8">
          
          <!-- Java Defaults Card -->
          <div class="bg-gray-900/40 backdrop-blur-sm border border-gray-800 rounded-2xl overflow-hidden p-6 hover:border-gray-700 transition-colors">
             <div class="flex items-center gap-4 mb-6 pb-6 border-b border-gray-800/50">
                <div class="p-3 bg-primary-500/10 rounded-xl ring-1 ring-primary-500/20 flex jsutify-center items-center">
                   <UIcon name="i-lucide-coffee" class="w-6 h-6 text-primary-400" />
                </div>
                <div>
                   <h3 class="text-xl font-bold text-white">Java Configuration</h3>
                   <p class="text-sm text-gray-400">Manage Java versions and default startup parameters</p>
                </div>
             </div>

             <div class="space-y-8">
                <!-- Java Status --> 
                <div class="p-4 rounded-xl border border-gray-800 bg-gray-950/50">
                    <div class="flex items-center justify-between mb-3">
                       <div class="flex items-center gap-3">
                          <h4 class="font-semibold text-gray-200">System Java Status</h4>
                          <UBadge :color="javaStatus.installed ? 'success' : 'warning'" variant="subtle" size="xs">
                             {{ javaStatus.installed ? 'Detected' : 'Not Found' }}
                          </UBadge>
                       </div>
                       <UButton size="xs" color="neutral" variant="ghost" :loading="checkingJava" @click="checkJava" icon="i-lucide-refresh-cw">
                          Refresh
                       </UButton>
                    </div>
                    
                    <div v-if="javaStatus.installed" class="flex items-center gap-2 text-sm text-gray-400 bg-gray-900/50 p-2 rounded-lg border border-gray-800/50 font-mono ">
                       <UIcon name="i-lucide-terminal" class="w-4 h-4 text-gray-500" />
                       {{ javaStatus.version }}
                    </div>
                    <div v-else class="flex items-start gap-3 p-3 bg-error-500/10 border border-error-500/20 rounded-lg">
                       <UIcon name="i-lucide-alert-circle" class="w-5 h-5 text-error-400 shrink-0 mt-0.5" />
                       <div class="text-sm text-error-400">
                          <p class="font-medium">Java not detected</p>
                          <p class="opacity-80 mt-1">Please ensure Java is installed and added to your system PATH, or manually configure paths below.</p>
                       </div>
                    </div>
                </div>

                <!-- Default RAM -->
                <div class="space-y-4">
                   <div class="flex justify-between items-center">
                     <div>
                        <label class="text-base font-medium text-white">Default Memory (RAM)</label>
                        <p class="text-xs text-gray-500 mt-1">Allocated memory for new servers</p>
                     </div>
                     <span class="text-2xl font-bold text-primary-400">{{ settings.defaultMemory }} <span class="text-sm text-gray-500 font-normal">GB</span></span>
                   </div>
                   <div class="px-2">
                      <USlider v-model="settings.defaultMemory" :min="1" :max="systemRamGB" :step="0.5" color="primary" size="lg" />
                   </div>
                   <div class="flex justify-between text-xs text-gray-500 font-mono px-1">
                      <span>1 GB</span>
                      <span>{{ systemRamGB }} GB (System Total)</span>
                   </div>
                </div>

                <!-- Default Flags -->
                <div class="space-y-3">
                   <div>
                      <label class="text-base font-medium text-white">Global Startup Flags</label>
                      <p class="text-xs text-gray-500 mt-1">Arguments added to every server launch command</p>
                   </div>
                   <UTextarea 
                      v-model="settings.defaultFlags" 
                      :rows="3" 
                      placeholder="-XX:+UseG1GC ..." 
                      variant="outline"
                      color="neutral"
                      class="font-mono text-sm w-full"
                   />
                </div>

                <!-- Install Java Section -->
                <div class="p-4 rounded-xl border border-primary-500/30 bg-primary-500/5 relative overflow-hidden">
                    <div class="flex items-center justify-between relative z-10">
                       <div>
                          <h4 class="font-semibold text-white flex items-center gap-2">
                             <UIcon name="i-lucide-download-cloud" class="w-4 h-4 text-primary-400" />
                             Install Java
                          </h4>
                          <p class="text-xs text-primary-200/70 mt-0.5">Download official Eclipse Temurin runtimes</p>
                       </div>
                       <div class="flex gap-2">
                          <UButton 
                             v-for="ver in [8, 17, 21]" 
                             :key="ver"
                             size="xs"
                             color="primary" 
                             variant="soft"
                             :loading="downloadingVersion === ver"
                             :disabled="downloadingVersion !== null && downloadingVersion !== ver"
                             @click="handleJavaDownload(ver)"
                          >
                             Java {{ ver }}
                          </UButton>
                       </div>
                    </div>
                    
                    <!-- Progress Overlay -->
                    <div v-if="downloadingVersion !== null" class="mt-4 space-y-2">
                        <div class="flex justify-between text-xs text-primary-200">
                           <span>Downloading Java {{ downloadingVersion }}...</span>
                           <span class="animate-pulse">Please wait</span>
                        </div>
                        <UProgress animation="carousel" color="primary" size="sm" />
                    </div>
                </div>
                
                <!-- Java Installations -->
                <div class="pt-6 border-t border-gray-800/50">
                    <div class="flex items-center justify-between mb-4">
                       <div>
                          <h4 class="font-semibold text-white">Java Installations</h4>
                          <p class="text-xs text-gray-500">Configure paths for specific Java versions</p>
                       </div>
                       <UButton size="sm" color="neutral" variant="soft" icon="i-lucide-scan" @click="detectAllJavaVersions" :loading="detectingJava">
                          Auto-Detect
                       </UButton>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                       <!-- Java 8 -->
                       <div class="p-4 bg-gray-950/30 rounded-xl border border-gray-800/50 group hover:border-gray-700 transition-colors">
                          <div class="flex items-center justify-between mb-3">
                             <div class="flex items-center gap-2">
                                <span class="font-medium text-gray-200">Java 8</span>
                                <UBadge color="neutral" variant="subtle" size="xs">MC 1.12.x-</UBadge>
                             </div>
                             <div v-if="settings.javaInstallations.java8" class="w-2 h-2 rounded-full bg-success-500"></div>
                          </div>
                          <UInput v-model="settings.javaInstallations.java8" class="w-full" placeholder="Path to java.exe" size="sm" color="neutral" variant="outline" :ui="{ trailing: 'pointer-events-auto' }">
                             <template #trailing>
                                <UButton color="neutral" variant="link" icon="i-lucide-folder" :padded="false" @click="browseJavaPath('java8')" />
                             </template>
                          </UInput>
                       </div>

                       <!-- Java 11 -->
                       <div class="p-4 bg-gray-950/30 rounded-xl border border-gray-800/50 group hover:border-gray-700 transition-colors">
                          <div class="flex items-center justify-between mb-3">
                             <div class="flex items-center gap-2">
                                <span class="font-medium text-gray-200">Java 11</span>
                                <UBadge color="neutral" variant="subtle" size="xs">MC 1.13-1.16</UBadge>
                             </div>
                             <div v-if="settings.javaInstallations.java11" class="w-2 h-2 rounded-full bg-success-500"></div>
                          </div>
                          <UInput v-model="settings.javaInstallations.java11" class="w-full" placeholder="Path to java.exe" size="sm" color="neutral" variant="outline" :ui="{ trailing: 'pointer-events-auto' }">
                             <template #trailing>
                                <UButton color="neutral" variant="link" icon="i-lucide-folder" :padded="false" @click="browseJavaPath('java11')" />
                             </template>
                          </UInput>
                       </div>

                       <!-- Java 17 -->
                       <div class="p-4 bg-gray-950/30 rounded-xl border border-gray-800/50 group hover:border-gray-700 transition-colors">
                          <div class="flex items-center justify-between mb-3">
                             <div class="flex items-center gap-2">
                                <span class="font-medium text-gray-200">Java 17</span>
                                <UBadge color="primary" variant="subtle" size="xs">MC 1.17-1.20.4</UBadge>
                             </div>
                             <div v-if="settings.javaInstallations.java17" class="w-2 h-2 rounded-full bg-success-500"></div>
                          </div>
                          <UInput v-model="settings.javaInstallations.java17" class="w-full" placeholder="Path to java.exe" size="sm" color="neutral" variant="outline" :ui="{ trailing: 'pointer-events-auto' }">
                             <template #trailing>
                                <UButton color="neutral" variant="link" icon="i-lucide-folder" :padded="false" @click="browseJavaPath('java17')" />
                             </template>
                          </UInput>
                       </div>

                       <!-- Java 21 -->
                       <div class="p-4 bg-gray-950/30 rounded-xl border border-gray-800/50 group hover:border-gray-700 transition-colors">
                          <div class="flex items-center justify-between mb-3">
                             <div class="flex items-center gap-2">
                                <span class="font-medium text-gray-200">Java 21</span>
                                <UBadge color="neutral" variant="subtle" size="xs">MC 1.20.5+</UBadge>
                             </div>
                             <div v-if="settings.javaInstallations.java21" class="w-2 h-2 rounded-full bg-success-500"></div>
                          </div>
                          <UInput v-model="settings.javaInstallations.java21" class="w-full" placeholder="Path to java.exe" size="sm" color="neutral" variant="outline" :ui="{ trailing: 'pointer-events-auto' }">
                             <template #trailing>
                                <UButton color="neutral" variant="link" icon="i-lucide-folder" :padded="false" @click="browseJavaPath('java21')" />
                             </template>
                          </UInput>
                       </div>
                    </div>
                </div>
             </div>
          </div>
          
          <!-- App Settings Card -->
          <div class="bg-gray-900/40 backdrop-blur-sm border border-gray-800 rounded-2xl overflow-hidden p-6 hover:border-gray-700 transition-colors">
             <div class="flex items-center gap-4 mb-6 pb-6 border-b border-gray-800/50">
                <div class="p-3 bg-neutral-500/10 rounded-xl ring-1 ring-neutral-500/20 flex jsutify-center items-center">
                   <UIcon name="i-lucide-monitor" class="w-6 h-6 text-neutral-400" />
                </div>
                <div>
                   <h3 class="text-xl font-bold text-white">Application</h3>
                   <p class="text-sm text-gray-400">Customize MineDash behavior</p>
                </div>
             </div>
             
             <div class="space-y-6">
                <div class="flex items-center justify-between p-4 bg-gray-950/30 rounded-xl border border-gray-800/50">
                   <div class="flex items-start gap-4">
                      <div class="p-2 bg-gray-900 rounded-lg flex jsutify-center items-center">
                         <UIcon name="i-lucide-minimize-2" class="w-5 h-5 text-gray-400" />
                      </div>
                      <div>
                         <h4 class="font-medium text-white">Minimize to Tray</h4>
                         <p class="text-xs text-gray-500 mt-1">Keep the application running in the background when closed</p>
                      </div>
                   </div>
                   <USwitch v-model="settings.minimizeOnClose" color="primary" size="lg" />
                </div>
             </div>
          </div>

          <!-- About Card -->
          <div class="bg-gray-900/40 backdrop-blur-sm border border-gray-800 rounded-2xl overflow-hidden p-6 hover:border-gray-700 transition-colors">
             <div class="flex items-center gap-4 mb-6 pb-6 border-b border-gray-800/50">
                <div class="p-3 bg-neutral-500/10 rounded-xl ring-1 ring-neutral-500/20 flex jsutify-center items-center">
                   <UIcon name="i-lucide-info" class="w-6 h-6 text-gray-400" />
                </div>
                <div>
                   <h3 class="text-xl font-bold text-white">About VoidLink</h3>
                   <p class="text-sm text-gray-400">Version information and updates</p>
                </div>
             </div>

             <div class="space-y-6">
                <div class="flex items-center justify-between">
                   <div class="space-y-1">
                      <p class="text-sm text-gray-400">Current Version: <span class="text-white font-mono ml-1">{{ currentVersion }}</span></p>
                      <p class="text-xs text-gray-500">Developed by <a href="http://makoto.com.pl" target="_blank" rel="noopener noreferrer" class="text-primary-400 hover:text-primary-300 hover:underline transition-colors">MakotoPD</a></p>
                   </div>
                   <UButton 
                      icon="i-lucide-refresh-cw" 
                      color="neutral" 
                      variant="soft" 
                      :loading="checkingUpdate"
                      @click="checkUpdate"
                   >
                      Check for Updates
                   </UButton>
                </div>
                
                <!-- Update Available Banner -->
                <div v-if="updateInfo?.available" class="p-6 bg-gradient-to-br from-primary-900/20 to-primary-900/5 border border-primary-500/30 rounded-xl relative overflow-hidden group">
                   <div class="absolute inset-0 bg-primary-500/5 group-hover:bg-primary-500/10 transition-colors"></div>
                   
                   <div class="relative z-10 flex items-start gap-4">
                      <div class="p-3 bg-primary-500/20 rounded-full animate-pulse flex jsutify-center items-center">
                         <UIcon name="i-lucide-download" class="w-6 h-6 text-primary-400" />
                      </div>
                      <div class="flex-1">
                         <h4 class="text-lg font-bold text-white mb-1">New Version Available!</h4>
                         <p class="text-primary-200 mb-4">
                            Update <strong>{{ updateInfo.latestVersion }}</strong> is ready to download. You are currently on <span class="opacity-75">{{ updateInfo.currentVersion }}</span>.
                         </p>
                         <UButton 
                            color="primary"
                            icon="i-lucide-external-link"
                            @click="openReleasePage"
                            class="shadow-lg shadow-primary-900/20"
                         >
                            Download Update
                         </UButton>
                      </div>
                   </div>
                </div>
                
                <!-- Up to date message -->
                <div v-else-if="updateInfo && !updateInfo.available" class="flex items-center gap-3 p-4 bg-success-500/5 border border-success-500/20 rounded-xl">
                   <div class="p-1 bg-success-500/10 rounded-full">
                      <UIcon name="i-lucide-check-circle" class="w-5 h-5 text-success-500" />
                   </div>
                   <span class="text-sm font-medium text-success-400">You are running the latest version of VoidLink.</span>
                </div>
             </div>
          </div>
          
          <div class="flex justify-end pt-4">
             <UButton color="primary" size="xl" :loading="saving" @click="saveSettings" icon="i-lucide-save" class="font-bold shadow-lg shadow-primary-500/20 px-8">Save All Settings</UButton>
          </div>
       </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { open } from '@tauri-apps/plugin-shell'
import { useJava } from '~/composables/useJava'
import { useUpdateChecker } from '~/composables/useUpdateChecker'
import { GITHUB_RELEASES_URL } from '~/utils/version'
import { useSettingsStore } from '~/stores/useSettingsStore'
import { storeToRefs } from 'pinia'

const loading = ref(true)
const saving = ref(false)
const checkingJava = ref(false)
const javaStatus = ref({ installed: false, version: '', details: '' })

const { installations, scanJava, validateJavaPath, downloadJava, fetchAdoptiumRelease } = useJava()

const downloadingVersion = ref<number | null>(null)
import { join, downloadDir, appDataDir } from '@tauri-apps/api/path'

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
   const url = updateInfo.value?.releaseUrl || GITHUB_RELEASES_URL
   if (url) {
      await open(url)
   }
}

async function checkJava() {
   checkingJava.value = true
   await scanJava()
   
   // Find the "best" system java (latest version)
   const latest = installations.value[0] // Sorted by backend
   
   if (latest && latest.is_valid) {
      javaStatus.value = {
         installed: true,
         version: `Java ${latest.major} (${latest.version})`,
         details: latest.path
      }
   } else {
      javaStatus.value = {
         installed: false,
         version: '',
         details: 'No Java detected'
      }
   }
   checkingJava.value = false
}

async function browseJavaPath(key: 'java8' | 'java11' | 'java17' | 'java21') {
   try {
      const selected = await openDialog({
         multiple: false,
         filters: [{
            name: 'Java',
            extensions: ['exe', '']
         }]
      })

      if (selected && typeof selected === 'string') {
         settings.value.javaInstallations[key] = selected
      }
   } catch (e) {
      console.error('Failed to open file dialog', e)
   }
}

async function detectAllJavaVersions() {
   detectingJava.value = true
   try {
      await scanJava()
      
      // Auto-assign known versions
      for (const java of installations.value) {
         if (!java.is_valid || !java.major) continue
         
         if (java.major === 8 && !settings.value.javaInstallations.java8) {
            settings.value.javaInstallations.java8 = java.path
         } else if (java.major === 11 && !settings.value.javaInstallations.java11) {
            settings.value.javaInstallations.java11 = java.path
         } else if (java.major === 17 && !settings.value.javaInstallations.java17) {
            settings.value.javaInstallations.java17 = java.path
         } else if (java.major >= 21 && !settings.value.javaInstallations.java21) {
             settings.value.javaInstallations.java21 = java.path
         }
      }
      
   } catch (e) {
      console.error('Failed to detect Java versions', e)
   } finally {
      detectingJava.value = false
   }
}

async function handleJavaDownload(major: number) {
   if (downloadingVersion.value !== null) return
   
   downloadingVersion.value = major
   try {
      // 1. Get Install Dir
      const appData = await appDataDir()
      const installDir = await join(appData, 'java', major.toString())
      
      console.log(`Downloading Java ${major} to ${installDir}`)
      
      // 2. Start Download
      const javaPath = await downloadJava(major, installDir)
      
      // 3. Update Settings
      if (major === 8) settings.value.javaInstallations.java8 = javaPath
      if (major === 11) settings.value.javaInstallations.java11 = javaPath
      if (major === 17) settings.value.javaInstallations.java17 = javaPath
      if (major === 21) settings.value.javaInstallations.java21 = javaPath
      
      // 4. Refresh List
      await scanJava()
      checkJava()
      
      // Notify
      // (Using console for now, maybe toast later if available)
      console.log('Java installed successfully!')
      
   } catch (e) {
      console.error('Download failed', e)
      // Error handling UI?
   } finally {
      downloadingVersion.value = null
   }
}

// Wrapper for manual save button (although auto-save is active)
async function handleManualSave() {
   saving.value = true
   await saveSettings()
   saving.value = false
}

</script>

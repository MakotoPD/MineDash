<template>
  <div class="min-h-screen flex flex-col p-8">
    <div>
      <div class="flex justify-between">
        <div>
          <h2 class="text-3xl font-semibold">Overview</h2>
          <p class="text-gray-500">System healthy.</p>
        </div>
        <div>
          <UButton color="primary" icon="i-lucide-plus" to="/create" class="font-bold">Create Server</UButton>
        </div>
      </div>

      <div class="grid grid-cols-3 gap-8 mt-4">
        <div class="flex flex-col gap-2 bg-gray-800/40 border border-gray-700 rounded-2xl p-4">
          <div class="p-2 bg-primary-400/20 flex justify-center items-center w-fit rounded-lg">
            <UIcon name="i-lucide-server" class="w-6 h-6 text-primary-400" />
          </div>
          <p class="text-gray-500 font-semibold">Active Servers</p>
          <p class="text-xl text-gray-400"><span class="text-2xl font-bold text-white">{{ activeServersCount }}</span>/{{ servers.length }}</p>
        </div>
        <div class="flex flex-col gap-2 bg-gray-800/40 border border-gray-700 rounded-2xl p-4">
          <div class="p-2 bg-error-400/20 flex justify-center items-center w-fit rounded-lg">
            <UIcon name="i-lucide-cpu" class="w-6 h-6 text-error-400" />
          </div>
          <p class="text-gray-500 font-semibold">Total CPU (servers)</p>
          <p class="text-2xl font-bold text-white">{{ totalCpuUsage.toFixed(1) }}%</p>
        </div>
        <div class="flex flex-col gap-2 bg-gray-800/40 border border-gray-700 rounded-2xl p-4">
          <div class="p-2 bg-success-400/20 flex justify-center items-center w-fit rounded-lg">
            <UIcon name="i-lucide-memory-stick" class="w-6 h-6 text-success-400" />
          </div>
          <p class="text-gray-500 font-semibold">Total RAM (servers)</p>
          <p class="text-xl text-gray-400"><span class="text-2xl font-bold text-white">{{ formatBytes(totalRamUsage) }}</span> / {{ formatBytes(systemInfo.total_memory_bytes) }}</p>
        </div>
      </div>
    </div>
    <div class="mt-8">
      <div>
        <h2 class="text-3xl font-semibold">Detailed Metrics</h2>
      </div>
      <div class="grid grid-cols-2 gap-8 mt-4">
        <div 
          v-for="server in servers" 
          :key="server.id" 
          class="flex gap-4 bg-gray-800/40 border border-gray-700 rounded-xl overflow-hidden cursor-pointer hover:ring-2 hover:ring-primary-500 transition-all"
          @click="navigateTo(`/server/${server.path.split('/').pop()}`)"
        >
          <div :class="'w-1 ' + 'bg-' + getStatusColor(getServerStatus(server.path.split('/').pop() || ''))"></div>
          <div class="flex-1 py-4 pr-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-lg bg-primary-900/30 flex items-center justify-center">
                  <UIcon :name="server.icon || 'i-lucide-box'" class="w-5 h-5 text-primary-400" />
                </div>
                <div>
                  <h3 class="font-medium text-white">{{ server.name }}</h3>
                  <p class="text-xs text-gray-500">{{ server.typeName }} {{ server.version }}</p>
                  <p :class="'text-xs text-' + getStatusColor(getServerStatus(server.path.split('/').pop() || ''))">⬤ {{ getServerStatus(server.path.split('/').pop() || '') }}</p>
                  <p class="text-xs text-gray-500 flex items-center gap-1"><UIcon name="i-lucide-network" class="w-3 h-3" /> {{ localIp }}:{{ server.port }}</p>
                </div>
              </div>
            </div>
            <div v-if="getServerStatus(server.path.split('/').pop() || '') === 'online'" class="border-t border-gray-700 mt-1">
              <div class="flex flex-col gap-3 mt-3 pt-2 px-1">
                <!-- CPU Progress -->
                <div>
                  <p class="text-gray-500 text-sm">CPU <span>({{ getServerCpu(server.path.split('/').pop() || '').toFixed(1) }}%)</span></p>
                  <div class="flex items-center gap-2">
                    <UIcon name="i-lucide-cpu" class="w-4 h-4 text-error-400 flex-shrink-0" />
                    <div class="flex-1 h-2 bg-gray-700 rounded-full overflow-hidden">
                      <div 
                        class="h-full bg-error-500 transition-all duration-300"
                        :style="{ width: Math.min(getServerCpu(server.path.split('/').pop() || ''), 100) + '%' }"
                      ></div>
                    </div>
                  </div>
                </div>
                
                <!-- RAM Progress -->
                <div>
                  <p class="text-gray-500 text-sm">RAM <span>({{ formatBytes(getServerMemory(server.path.split('/').pop() || '')) }} / {{ server.javaSettings?.memory || 4 }}GB)</span></p>
                  <div class="flex items-center gap-2">
                    <UIcon name="i-lucide-memory-stick" class="w-4 h-4 text-success-400 flex-shrink-0" />
                    <div class="flex-1 h-2 bg-gray-700 rounded-full overflow-hidden">
                      <div 
                        class="h-full bg-success-500 transition-all duration-300"
                        :style="{ width: Math.min(getRamPercent(server), 100) + '%' }"
                      ></div>
                    </div>
                  </div>
                </div>
                
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="servers.length === 0" class="col-span-2 text-center py-8 text-gray-500">
          <UIcon name="i-lucide-server" class="w-12 h-12 mx-auto mb-2 opacity-30" />
          <p>No servers yet. Create one to get started!</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useServersStore } from '~/stores/useServersStore'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// Page meta for transitions later if needed
definePageMeta({
  layout: 'default'
})

// Use centralized servers store
const serversStore = useServersStore()
const { servers, loading } = storeToRefs(serversStore)
const { loadServers, getServerStatus, getStatusColor, getServerMemory, getServerCpu, formatBytes } = serversStore

// Process store for refreshing memory info
const processStore = useServerProcessStore()

// System info
interface SystemInfo {
  total_memory_bytes: number
  used_memory_bytes: number
  cpu_count: number
}
const systemInfo = ref<SystemInfo>({ total_memory_bytes: 0, used_memory_bytes: 0, cpu_count: 0 })
const localIp = ref('127.0.0.1')

let pollInterval: ReturnType<typeof setInterval> | null = null

onMounted(async () => {
  await loadServers()
  
  // Fetch system info and local IP
  try {
    systemInfo.value = await invoke<SystemInfo>('get_system_info')
    localIp.value = await invoke<string>('get_local_ip')
  } catch (e) {
    console.error('Failed to get system info:', e)
  }
  
  // Poll process info every 2 seconds for running servers
  pollInterval = setInterval(() => {
    for (const server of servers.value) {
      const serverId = server.path.split('/').pop() || ''
      if (getServerStatus(serverId) === 'online') {
        processStore.refreshProcessInfo(serverId)
      }
    }
  }, 2000)
})

onUnmounted(() => {
  if (pollInterval) clearInterval(pollInterval)
})

// Computed stats for overview cards
const activeServersCount = computed(() => {
  return servers.value.filter(s => getServerStatus(s.path.split('/').pop() || '') === 'online').length
})

const totalCpuUsage = computed(() => {
  return servers.value.reduce((total, server) => {
    const serverId = server.path.split('/').pop() || ''
    if (getServerStatus(serverId) === 'online') {
      return total + getServerCpu(serverId)
    }
    return total
  }, 0)
})

const totalRamUsage = computed(() => {
  return servers.value.reduce((total, server) => {
    const serverId = server.path.split('/').pop() || ''
    if (getServerStatus(serverId) === 'online') {
      return total + getServerMemory(serverId)
    }
    return total
  }, 0)
})

// Calculate RAM usage percentage based on server memory limit
function getRamPercent(server: { path: string; javaSettings?: { memory: number } }): number {
  const serverId = server.path.split('/').pop() || ''
  const memoryBytes = getServerMemory(serverId)
  const limitGB = server.javaSettings?.memory || 4
  const limitBytes = limitGB * 1024 * 1024 * 1024
  return (memoryBytes / limitBytes) * 100
}
</script>

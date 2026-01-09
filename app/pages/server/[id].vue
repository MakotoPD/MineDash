<template>
  <div class="min-h-screen bg-gray-950 flex flex-col">
    <!-- Header -->
    <header class="relative bg-gray-900 border-b border-gray-800 p-6 overflow-hidden">
       <!-- Gradient Background Mesh -->
       <div class="absolute inset-0 z-0">
          <div class="absolute top-0 right-0 w-96 h-96 bg-primary-500/10 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2"></div>
          <div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/5 rounded-full blur-3xl translate-y-1/2 -translate-x-1/4"></div>
       </div>

       <div class="relative z-10 max-w-7xl mx-auto flex flex-col md:flex-row md:items-center justify-between gap-6">
          <div class="flex items-center gap-6">
            <UButton
              icon="i-lucide-arrow-left"
              color="neutral"
              variant="ghost"
              to="/"
              class="hidden md:flex"
            />
            
            <div v-if="loading" class="h-16 w-64 bg-gray-800 animate-pulse rounded-xl" />
            <div v-else class="flex items-center gap-5">
               <div class="relative group cursor-pointer" @click="changeServerIcon">
                   <div class="absolute inset-0 bg-primary-500/20 rounded-2xl blur-lg group-hover:bg-primary-500/30 transition-all duration-500"></div>
                   <div class="relative w-16 h-16 rounded-2xl bg-gray-900 border border-gray-700 flex items-center justify-center shadow-2xl group-hover:scale-105 transition-transform duration-300 overflow-hidden">
                     <img v-if="serverIconUrl" :src="serverIconUrl" class="w-full h-full object-cover" />
                     <UIcon v-else :name="server?.icon || 'i-lucide-box'" class="w-8 h-8 text-primary-400 group-hover:text-primary-300 transition-colors" />
                     
                     <!-- Overlay for change hint -->
                     <div class="absolute inset-0 bg-black/50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                        <UIcon name="i-lucide-edit-2" class="w-4 h-4 text-white" />
                     </div>
                   </div>
                   <!-- Status Indicator Dot -->
                   <div class="absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-gray-900 shadow-sm" :class="statusBgClass"></div>
               </div>
               
               <div>
                  <div class="flex items-center gap-3">
                     <h1 class="font-bold text-2xl text-white tracking-tight">{{ server?.name }}</h1>
                     <UBadge :color="statusColor" variant="subtle" size="xs" class="uppercase font-bold tracking-wider">{{ serverStatus }}</UBadge>
                  </div>
                 <div class="flex items-center gap-2 mt-1 text-sm text-gray-400">
                    <span class="flex items-center gap-1.5 bg-gray-800/50 px-2 py-0.5 rounded-md border border-gray-700/50">
                        <UIcon name="i-lucide-layers" class="w-3.5 h-3.5" />
                        {{ server?.typeName }}{{ server?.modpack?.loader ? ` (${server.modpack.loader})` : '' }}
                    </span>
                    <span class="hidden sm:inline text-gray-600">•</span>
                    <span class="bg-gray-800/50 px-2 py-0.5 rounded-md border border-gray-700/50 font-mono text-xs">v{{ server?.version }}</span>
                    <span class="hidden sm:inline text-gray-600">•</span>
                    <span class="text-xs text-gray-500 font-mono cursor-pointer hover:text-white transition-colors" title="Click to copy ID" @click="copyId">#{{ server?.id }}</span>
                 </div>
               </div>
            </div>
          </div>

          <!-- Actions Toolbar -->
          <div class="flex items-center gap-3 bg-gray-800/30 p-1.5 rounded-xl border border-gray-700/50 backdrop-blur-sm shadow-sm">
             
             <!-- Primary Power Controls -->
             <div class="flex gap-1">
                 <UButton 
                   v-if="serverStatus === 'offline'"
                   color="success" 
                   icon="i-lucide-play" 
                   label="Start Server"
                   size="md"
                   @click="startServer"
                   class="shadow-lg shadow-green-500/10 hover:shadow-green-500/20 transition-all font-semibold px-6"
                 />
                 <template v-else>
                    <UButton 
                       color="error" 
                       icon="i-lucide-square" 
                       label="Stop" 
                       size="md"
                       variant="soft"
                       :loading="serverStatus === 'stopping'"
                       @click="stopServer" 
                    />
                    <UTooltip text="Force Kill">
                       <UButton 
                          color="error" 
                          variant="ghost"
                          icon="i-lucide-skull" 
                          size="md"
                          @click="killServer"
                       />
                    </UTooltip>
                 </template>
             </div>

             <div class="w-px h-8 bg-gray-700/50 mx-1"></div>

             <!-- Secondary Actions -->
             <UTooltip text="Open Server Folder">
                 <UButton 
                   icon="i-lucide-folder-open" 
                   color="neutral" 
                   variant="ghost" 
                   size="md"
                   @click="openServerFolder"
                 />
             </UTooltip>
             
              <!-- Update Modpack Button -->
              <UButton 
                 v-if="server?.modpack?.id && server.modpack.id !== 'custom'"
                 :loading="checkingUpdate"
                 icon="i-lucide-refresh-cw"
                 color="primary"
                 variant="ghost"
                 size="md"
                 @click="checkModpackUpdate"
                 :title="updateAvailable ? 'Update Available' : 'Check Updates'"
              >
                  <span v-if="updateAvailable" class="relative flex h-2 w-2 mr-1">
                    <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary-400 opacity-75"></span>
                    <span class="relative inline-flex rounded-full h-2 w-2 bg-primary-500"></span>
                  </span>
              </UButton>
           </div>
       </div>
    </header>

    <!-- Update Modal -->
    <UModal v-model:open="showUpdateModal">
      <template #body>
         <div class="p-6">
            <h3 class="text-xl font-bold mb-4">Update Modpack</h3>
            <p class="text-gray-400 mb-4">
               A new version is available: <span class="text-white font-bold">{{ updateData?.name }}</span>
            </p>
            
            <div class="bg-yellow-500/10 border border-yellow-500/20 p-3 rounded mb-4 text-sm text-yellow-500">
               Warning: Updating will replace mods and config files. Your world data will be safe, but custom config changes might be lost.
            </div>

            <div class="flex justify-end gap-2">
               <UButton color="neutral" variant="ghost" @click="showUpdateModal = false">Cancel</UButton>
               <UButton color="primary" :loading="isUpdating" @click="performModpackUpdate">Update Now</UButton>
            </div>
         </div>
      </template>
    </UModal>


    <!-- Main Content -->
    <main class="flex-1 overflow-hidden">
      <div v-if="loading" class="flex items-center justify-center h-full">
         <UIcon name="i-lucide-loader-2" class="w-8 h-8 animate-spin text-primary-500" />
      </div>

      <div v-else class="max-w-7xl mx-auto h-full p-6 flex flex-col">
        <UTabs v-model="selectedTab" variant="pill" :items="tabs" class="w-full flex-1 flex flex-col">
           
           <!-- Performance -->
           <template #performance>
             <ServerPerformance 
                v-if="server" 
                :server-id="storeServerId" 
                :server-data="server" 
             />
           </template>

           <!-- Console / Overview -->
           <template #console>
             <div class="h-full flex flex-col">
               <div class="relative">
                  <div ref="consoleRef" class="flex-1 bg-black max-h-[68vh] rounded-lg border border-gray-800 p-4 font-mono text-xs md:text-sm text-gray-300 overflow-y-auto custom-scrollbar font-ligatures-none">
                     <div v-if="consoleLines.length === 0" class="text-gray-500 italic">Server is offline. Output will appear here.</div>
                     
                     <div 
                        v-for="(line, i) in consoleLines" 
                        :key="i" 
                        class="flex items-start gap-2 py-0.5 group hover:bg-gray-800/30 px-1 -mx-1 rounded"
                     >
                        <!-- Parsed log line -->
                        <template v-if="parseLogLine(line)">
                           <span class="text-gray-500 shrink-0">{{ parseLogLine(line).time }}</span>
                           <UBadge 
                              :color="getLogLevelColor(parseLogLine(line).level)" 
                              variant="subtle" 
                              size="xs"
                              class="shrink-0 w-12 justify-center"
                           >{{ parseLogLine(line).level }}</UBadge>
                           <span 
                              class="whitespace-pre-wrap break-all flex-1"
                              :class="{
                                 'text-yellow-300': parseLogLine(line).level === 'WARN',
                                 'text-red-400': parseLogLine(line).level === 'ERROR'
                              }"
                              v-html="parseAnsiToHtml(parseLogLine(line).message)"
                           ></span>
                        </template>
                        
                        <!-- Raw line (no pattern match) -->
                        <template v-else>
                           <span class="whitespace-pre-wrap break-all flex-1" v-html="parseAnsiToHtml(line)"></span>
                        </template>
                     </div>
                  </div>
                  
                  <!-- Scroll to bottom button -->
                  <UButton 
                     class="absolute bottom-3 right-3 opacity-70 hover:opacity-100"
                     icon="i-lucide-arrow-down" 
                     color="neutral" 
                     variant="solid"
                     size="sm"
                     @click="scrollToBottom"
                  />
               </div>
               <div class="mt-4 flex gap-2">
                 <UInput 
                    v-model="consoleInput"
                    placeholder="Type a command..." 
                    class="flex-1" 
                    icon="i-lucide-terminal" 
                    @keydown.enter="sendCommand"
                    :disabled="serverStatus === 'offline'"
                 />
                 <UButton 
                    color="neutral" 
                    variant="soft" 
                    @click="sendCommand"
                    :disabled="serverStatus === 'offline'"
                 >Send</UButton>
               </div>
             </div>
           </template>

           <!-- Settings (RAM, Flags, Java) -->
           <template #settings>
              <div class="max-w-6xl mx-auto space-y-6 py-6 px-4 pb-24">
                 
                 <!-- Header & Actions -->
                  <div class="flex flex-col md:flex-row justify-between items-start md:items-center bg-gray-900/80 border border-gray-800 p-4 rounded-xl shadow-lg sticky top-0 z-20 backdrop-blur-xl gap-4">
                    <div>
                        <h2 class="font-bold text-xl text-white flex items-center gap-2">
                           <UIcon name="i-lucide-settings-2" class="w-6 h-6 text-primary-500" />
                           Server Configuration
                        </h2>
                       <p class="text-sm text-gray-400">Manage general settings, gameplay, and system performance</p>
                    </div>
                    <div class="flex gap-3 items-center w-full md:w-auto">
                        <UButton 
                           size="md" 
                           color="neutral" 
                           variant="outline" 
                           icon="i-lucide-file-code" 
                           @click="showPropertiesEditor = true"
                           class="flex-1 md:flex-none justify-center"
                        >
                           Edit properties
                        </UButton>
                        <UButton 
                           size="md" 
                           color="primary" 
                           icon="i-lucide-save" 
                           :loading="saving" 
                           @click="saveAllSettings"
                           class="flex-1 md:flex-none justify-center shadow-lg shadow-primary-500/20"
                        >
                           Save Changes
                        </UButton>
                    </div>
                 </div>

                 <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
                    
                    <!-- Left Column: General & Gameplay -->
                    <div class="space-y-6">
                       <!-- General Settings -->
                       <UCard :ui="{ root: 'bg-gray-900/50 backdrop-blur-sm ring-1 ring-gray-800' }">
                          <template #header>
                             <div class="flex items-center gap-3">
                                <div class="p-2 bg-primary-500/10 rounded-lg flex items-center justify-center">
                                   <UIcon name="i-lucide-sliders" class="w-5 h-5 text-primary-500" />
                                </div>
                                <div>
                                   <h3 class="font-bold text-white">General Information</h3>
                                   <p class="text-xs text-gray-500">Basic server details</p>
                                </div>
                             </div>
                          </template>
                          
                          <div class="space-y-6">
                             <div class="space-y-2 flex flex-col gap-1">
                                <label class="text-sm font-medium text-gray-300">Server Name</label>
                                <UInput 
                                   v-model="serverName" 
                                   placeholder="My Awesome Server" 
                                   icon="i-lucide-pencil" 
                                   size="lg"
                                  
                                />
                             </div>

                             <!-- MOTD Editor -->
                             <div class="space-y-3">
                                <label class="block text-sm font-medium text-gray-300">Message of the Day (MOTD)</label>
                                <div class="flex flex-wrap gap-1.5 mb-2 bg-gray-950/30 p-2 rounded-lg border border-gray-800/50">
                                   <!-- Colors -->
                                   <button v-for="code in mcColorCodes" :key="code.code" 
                                      @click="insertMotdCode(code.code)"
                                      class="w-6 h-6 rounded text-[10px] font-bold border border-white/10 hover:scale-110 transition-transform shadow-sm"
                                      :style="{ backgroundColor: code.color, color: code.textColor }"
                                      :title="code.name"
                                   >
                                      {{ code.code }}
                                   </button>
                                   <div class="w-px bg-gray-700 mx-1 h-6"></div>
                                   <!-- Styles -->
                                   <button v-for="style in mcStyleCodes" :key="style.code"
                                      @click="insertMotdCode(style.code)"
                                      class="px-2 h-6 rounded text-[10px] bg-gray-800 border border-gray-700 hover:bg-gray-700 transition-colors font-mono"
                                      :class="style.class"
                                      :title="style.name"
                                   >
                                      {{ style.label }}
                                   </button>
                                </div>
                                <textarea 
                                   ref="motdTextarea"
                                   :value="getPropertyValue('motd')" 
                                   @input="(e) => updateProperty('motd', (e.target as HTMLTextAreaElement).value)"
                                   placeholder="A Minecraft Server"
                                   class="w-full px-4 py-3 bg-gray-950/50 border border-gray-800 rounded-xl text-white font-mono text-sm resize-none focus:ring-2 focus:ring-primary-500/50 focus:border-primary-500 transition-all outline-none"
                                   rows="3"
                                ></textarea>
                                <div class="p-4 bg-black/80 border border-gray-800 rounded-xl overflow-hidden relative group">
                                   <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                                      <UBadge size="xs" color="neutral" variant="subtle">PREVIEW</UBadge>
                                   </div>
                                   <div class="font-minecraft text-lg leading-tight flex items-center gap-2">
                                      <img v-if="serverIconUrl" :src="serverIconUrl" class="w-16 h-16 rounded-sm opacity-90" />
                                      <UIcon v-else name="i-lucide-box" class="w-16 h-16 rounded-sm bg-gray-800 text-gray-700 p-4" />
                                      <div class="flex flex-col justify-center h-16">
                                         <div class="text-white font-minecraft">{{ serverName || 'Minecraft Server' }}</div>
                                         <div v-html="renderMotdPreview(getPropertyValue('motd'))"></div>
                                      </div>
                                   </div>
                                </div>
                             </div>
                          </div>
                       </UCard>

                       <!-- Gameplay Settings -->
                       <UCard :ui="{ root: 'bg-gray-900/50 backdrop-blur-sm ring-1 ring-gray-800' }">
                          <template #header>
                             <div class="flex items-center gap-3">
                                <div class="p-2 bg-emerald-500/10 rounded-lg flex items-center justify-center">
                                   <UIcon name="i-lucide-gamepad-2" class="w-5 h-5 text-emerald-500" />
                                </div>
                                <div>
                                   <h3 class="font-bold text-white">Gameplay Experience</h3>
                                   <p class="text-xs text-gray-500">Game rules and mechanics</p>
                                </div>
                             </div>
                          </template>

                          <div class="space-y-6">
                             <div class="grid grid-cols-2 gap-4">
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">Gamemode</label>
                                   <USelectMenu 
                                      class="w-full"
                                      size="lg"
                                      :model-value="getPropertyValue('gamemode')" 
                                      @update:model-value="(val) => updateProperty('gamemode', val)"
                                      :items="['survival', 'creative', 'adventure', 'spectator']"
                                      icon="i-lucide-swords"
                                   />
                                </div>
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">Difficulty</label>
                                   <USelectMenu 
                                      class="w-full"
                                      size="lg"
                                      :model-value="getPropertyValue('difficulty')" 
                                      @update:model-value="(val) => updateProperty('difficulty', val)"
                                      :items="['peaceful', 'easy', 'normal', 'hard']"
                                      icon="i-lucide-skull"
                                   />
                                </div>
                             </div>
                             
                             <div class="space-y-2">
                                <label class="block text-sm font-medium text-gray-300">Server Port</label>
                                <UInput 
                                   type="number"
                                   placeholder="25565"
                                   class="w-full"
                                   size="lg"
                                   icon="i-lucide-network"
                                   :model-value="getPropertyValue('server-port')" 
                                   @update:model-value="(val) => updateProperty('server-port', val)"
                                   :ui="{ icon: { leading: { pointer: '' } } }"
                                />
                             </div>

                             <div class="space-y-3 pt-2">
                                <div v-for="(item, idx) in [
                                   { label: 'PVP Combat', desc: 'Allow players to fight each other', prop: 'pvp', icon: 'i-lucide-swords', color: 'text-red-400' },
                                   { label: 'Allow Flight', desc: 'Allow flying in survival mode', prop: 'allow-flight', icon: 'i-lucide-cloud', color: 'text-sky-400' },
                                   { label: 'Command Blocks', desc: 'Enable command block functionality', prop: 'enable-command-block', icon: 'i-lucide-box-select', color: 'text-purple-400' },
                                   { label: 'Hardcore Mode', desc: 'Players are banned upon death', prop: 'hardcore', icon: 'i-lucide-skull', color: 'text-rose-500' }
                                ]" :key="idx"
                                class="flex items-center justify-between p-4 bg-gray-800/30 border border-gray-800/50 rounded-xl hover:bg-gray-800/50 transition-colors group">
                                   <div class="flex items-center gap-3">
                                      <div class="p-2 rounded-lg bg-gray-900 group-hover:bg-gray-800 flex items-center justify-center transition-colors">
                                         <UIcon :name="item.icon" class="w-5 h-5" :class="item.color" />
                                      </div>
                                      <div>
                                         <p class="font-medium text-white">{{ item.label }}</p>
                                         <p class="text-xs text-gray-500">{{ item.desc }}</p>
                                      </div>
                                   </div>
                                   <USwitch 
                                      :model-value="getPropertyValue(item.prop) === 'true'" 
                                      @update:model-value="(val) => updateProperty(item.prop, val)" 
                                      color="primary"
                                      size="lg"
                                   />
                                </div>
                             </div>
                          </div>
                       </UCard>
                    </div>

                    <!-- Right Column: System & Danger -->
                    <div class="space-y-6">
                       <!-- System & Performance -->
                       <UCard :ui="{ root: 'bg-gray-900/50 backdrop-blur-sm ring-1 ring-gray-800' }">
                          <template #header>
                             <div class="flex items-center gap-3">
                                <div class="p-2 bg-blue-500/10 rounded-lg flex items-center justify-center">
                                   <UIcon name="i-lucide-cpu" class="w-5 h-5 text-blue-500" />
                                </div>
                                <div>
                                   <h3 class="font-bold text-white">System & Performance</h3>
                                   <p class="text-xs text-gray-500">Allocations and limits</p>
                                </div>
                             </div>
                          </template>
                          
                          <div class="space-y-8">
                             <!-- Memory Slider -->
                             <div class="space-y-4 bg-gray-800/20 p-5 rounded-xl border border-gray-800/50">
                                <div class="flex justify-between items-end">
                                  <div class="flex flex-col">
                                     <label class="text-sm font-medium text-gray-200 flex items-center gap-2">
                                        <UIcon name="i-lucide-memory-stick" class="w-4 h-4 text-primary-400" />
                                        RAM Allocation
                                     </label>
                                     <span class="text-xs text-gray-500">Reserved memory for Java</span>
                                  </div>
                                  <span class="text-2xl font-bold text-primary-400 tracking-tight">{{ javaSettings.memory }} <span class="text-sm font-normal text-gray-500">GB</span></span>
                                </div>
                                <USlider v-model="javaSettings.memory" :min="1" :max="systemRamGB" :step="0.5" color="primary" class="w-full" />
                                <div class="flex justify-between text-xs text-gray-500 font-mono">
                                   <span>1 GB</span>
                                   <span>{{ systemRamGB }} GB</span>
                                </div>
                             </div>

                             <!-- Grid Inputs -->
                             <div class="grid grid-cols-2 gap-4">
                                <!-- Max Players -->
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">Max Players</label>
                                   <UInputNumber 
                                      size="lg" 
                                      class="w-full" 
                                      placeholder="20" 
                                      :value="getPropertyValue('max-players')" 
                                      @update:model-value="(val) => updateProperty('max-players', val)"
                                   />
                                </div>

                                <!-- View Distance -->
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">View Distance</label>
                                   <UInputNumber 
                                      size="lg" 
                                      class="w-full" 
                                      placeholder="10" 
                                      :min="2" 
                                      :max="32" 
                                      :value="getPropertyValue('view-distance')" 
                                      @update:model-value="(val) => updateProperty('view-distance', val)"
                                   />
                                </div>
                                
                                <!-- Sim Distance -->
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">Sim Distance</label>
                                   <UInputNumber 
                                      size="lg" 
                                      class="w-full" 
                                      placeholder="10" 
                                      :min="2" 
                                      :max="32" 
                                      :value="getPropertyValue('simulation-distance')" 
                                      @update:model-value="(val) => updateProperty('simulation-distance', val)"
                                   />
                                </div>

                                <!-- Spawn Protection -->
                                <div class="space-y-2">
                                   <label class="block text-sm font-medium text-gray-300">Spawn Radius</label>
                                   <UInputNumber 
                                      size="lg" 
                                      class="w-full" 
                                      placeholder="16" 
                                      :value="getPropertyValue('spawn-protection')" 
                                      @update:model-value="(val) => updateProperty('spawn-protection', val)"
                                   />
                                </div>
                             </div>

                             <div class="space-y-2 flex flex-col">
                                <label class="text-sm font-medium text-gray-300 flex items-center gap-2">
                                   <UIcon name="i-lucide-code-2" class="w-4 h-4" />
                                   Java Startup Flags
                                </label>
                                <UTextarea 
                                   v-model="javaSettings.flags" 
                                   placeholder="-Aikars flags..." 
                                   :rows="3" 
                                   class="font-mono text-xs"
                                   variant="outline"
                                   color="neutral" 
                                />
                             </div>
                          </div>
                       </UCard>

                       <!-- Security -->
                       <UCard :ui="{ root: 'bg-gray-900/50 backdrop-blur-sm ring-1 ring-gray-800' }">
                           <template #header>
                             <div class="flex items-center gap-3">
                                <div class="p-2 bg-indigo-500/10 rounded-lg flex items-center justify-center">
                                   <UIcon name="i-lucide-shield-check" class="w-5 h-5 text-indigo-500" />
                                </div>
                                <div>
                                   <h3 class="font-bold text-white">Access Control</h3>
                                   <p class="text-xs text-gray-500">Whitelist and verification</p>
                                </div>
                             </div>
                          </template>
                           <div class="space-y-3">
                              <div v-for="(item, idx) in [
                                 { label: 'Enable Whitelist', desc: 'Only whitelisted players can join', prop: 'white-list', icon: 'i-lucide-list-checks' },
                                 { label: 'Enforce Whitelist', desc: 'Kick non-whitelisted players on reload', prop: 'enforce-whitelist', icon: 'i-lucide-gavel' },
                                 { label: 'Online Mode', desc: 'Verify player accounts with Mojang', prop: 'online-mode', icon: 'i-lucide-globe-lock' }
                              ]" :key="idx"
                              class="flex items-center justify-between p-4 bg-gray-800/30 border border-gray-800/50 rounded-xl transition-colors"
                              :class="[
                                 (item.prop === 'enforce-whitelist' && getPropertyValue('white-list') !== 'true') ? 'opacity-50 cursor-not-allowed' : 'hover:bg-gray-800/50'
                              ]">
                                 <div class="flex items-center gap-3">
                                    <UIcon :name="item.icon" class="w-5 h-5 text-gray-400" />
                                    <div>
                                       <p class="font-medium text-white">{{ item.label }}</p>
                                       <p class="text-xs text-gray-500">{{ item.desc }}</p>
                                    </div>
                                 </div>
                                 <USwitch 
                                    :model-value="getPropertyValue(item.prop) === 'true'" 
                                    @update:model-value="(val) => updateAccessProperty(item.prop, val)" 
                                    :disabled="item.prop === 'enforce-whitelist' && getPropertyValue('white-list') !== 'true'"
                                    color="primary"
                                    size="lg"
                                 />
                              </div>
                           </div>
                       </UCard>

                       <!-- Danger Zone -->
                       <div class="relative overflow-hidden rounded-xl border border-error-900/50 bg-error-950/10 group">
                          <div class="absolute inset-0 bg-gradient-to-r from-error-900/10 to-transparent pointer-events-none"></div>
                          <div class="p-6 relative z-10">
                              <h3 class="font-bold text-error-500 flex items-center gap-2 mb-4">
                                 <UIcon name="i-lucide-alert-octagon" class="w-5 h-5" />
                                 Danger Zone
                              </h3>
                              
                              <div class="flex items-center justify-between bg-error-950/30 gap-4 p-4 rounded-lg border border-error-900/30 backdrop-blur-sm transition-colors hover:border-error-700/50">
                                 <div>
                                    <div class="font-bold text-white text-sm">Delete Server</div>
                                    <div class="text-xs text-gray-400 mt-1 max-w-[200px] sm:max-w-xs">Permanently delete this server and all its files. <br/> Cannot be undone.</div>
                                 </div>
                                 <UButton 
                                    color="error" 
                                    variant="solid" 
                                    label="Delete Server" 
                                    icon="i-lucide-trash-2"
                                    @click="openDeleteModal"
                                    class="shadow-lg shadow-error-500/20"
                                 />
                              </div>
                           </div>
                       </div>
                    </div>

                 </div>

                  <!-- Delete Confirmation Modal -->
                  <UModal v-model:open="showDeleteModal" title="Delete server" description="Are you sure you want to delete?">
                     <template #body>
                        <div class="p-6 space-y-4">
                           <div class="flex items-center gap-3 text-error-500 mb-2">
                              <div class="p-3 flex justify-center items-center bg-error-950 rounded-full ring-4 ring-error-900/30">
                                 <UIcon name="i-lucide-alert-triangle" class="w-6 h-6" />
                              </div>
                              <div>
                                 <h3 class="font-bold text-lg text-white">Delete Server?</h3>
                                 <p class="text-xs text-error-400 font-medium">This action is irreversible</p>
                              </div>
                           </div>
                           
                           <p class="text-gray-300 text-sm leading-relaxed bg-gray-900/50 p-4 rounded-lg border border-gray-800">
                              Are you sure you want to delete <span class="font-bold text-white">{{ serverFolderName }}</span>? 
                              This action will permanently remove all worlds, configurations, and player data.
                           </p>
                           
                           <div class="space-y-2 flex flex-col pt-2">
                              <label class="text-xs font-semibold text-gray-500 uppercase tracking-wider">Type server name to confirm</label>
                              <UInput 
                                 v-model="deleteConfirmation" 
                                 :placeholder="serverFolderName" 
                                 icon="i-lucide-trash-2"
                                 :ui="{ icon: { trailing: { pointer: '' } } }"
                                 color="error"
                                 size="lg"
                                 class="font-mono"
                              />
                           </div>
                        </div>
                     </template>

                     <template #footer>
                        <div class="flex justify-end gap-3 p-4 bg-gray-900/50 border-t border-gray-800">
                           <UButton color="neutral" variant="ghost" @click="showDeleteModal = false">Cancel</UButton>
                           <UButton 
                              color="error" 
                              variant="solid" 
                              label="Delete Permanently" 
                              icon="i-lucide-trash-2"
                              :loading="deletingServer"
                              :disabled="deleteConfirmation !== serverFolderName"
                              @click="confirmDeleteServer"
                           />
                        </div>
                     </template>
                     
                  </UModal>
              </div>
           </template>

           <template #crash-reports>
              <div class="h-full flex flex-col p-6 max-w-6xl mx-auto w-full">
                  <div class="flex items-center justify-between mb-6">
                     <div>
                        <h2 class="text-2xl font-bold text-white flex items-center gap-3">
                           <UIcon name="i-lucide-file-warning" class="w-8 h-8 text-red-500" />
                           Crash Reports
                        </h2>
                        <p class="text-gray-400">View and analyze server crashes</p>
                     </div>
                     <UButton icon="i-lucide-refresh-cw" color="neutral" variant="soft" @click="loadCrashReports" :loading="loadingReports">Refresh</UButton>
                  </div>

                  <div v-if="loadingReports" class="flex-1 flex justify-center items-center">
                      <UIcon name="i-lucide-loader-2" class="w-8 h-8 animate-spin text-primary-500" />
                  </div>

                  <div v-else-if="crashReports.length === 0" class="flex-1 flex flex-col justify-center items-center text-gray-500 border-2 border-dashed border-gray-800 rounded-xl bg-gray-900/20 p-12">
                      <UIcon name="i-lucide-check-circle-2" class="w-16 h-16 text-success-500/20 mb-4" />
                      <h3 class="text-xl font-bold text-gray-300">No Crashes Found</h3>
                      <p class="text-sm">Your server appears to be stable.</p>
                  </div>

                  <div v-else class="grid gap-4">
                      <div 
                         v-for="report in crashReports" 
                         :key="report.name"
                         class="p-4 bg-gray-900/50 border border-gray-800 rounded-xl hover:bg-gray-800/50 hover:border-gray-700 transition-all cursor-pointer group"
                         @click="openCrashReport(report)"
                      >
                         <div class="flex items-center justify-between">
                            <div class="flex items-center gap-4">
                               <div class="p-3 bg-red-500/10 rounded-lg group-hover:bg-red-500/20 transition-colors">
                                  <UIcon name="i-lucide-file-text" class="w-6 h-6 text-red-400" />
                               </div>
                               <div>
                                  <h4 class="font-bold text-white group-hover:text-primary-400 transition-colors">{{ report.name }}</h4>
                                  <p class="text-xs text-gray-500 font-mono">{{ new Date(report.created * 1000).toLocaleString() }}</p>
                               </div>
                            </div>
                            <UIcon name="i-lucide-chevron-right" class="w-5 h-5 text-gray-600 group-hover:text-white" />
                         </div>
                      </div>
                  </div>

                  <!-- Viewing Modal -->
                  <UModal v-model:open="showReportModal" fullscreen>
                      <template #header>
                         <div class="flex items-center justify-between p-4 border-b border-gray-800 bg-gray-900">
                             <h3 class="font-bold text-white flex items-center gap-2">
                                <UIcon name="i-lucide-file-warning" class="w-5 h-5 text-red-500" />
                                {{ viewingReport?.name }}
                             </h3>
                             <UButton icon="i-lucide-x" color="neutral" variant="ghost" @click="showReportModal = false" />
                         </div>
                      </template>
                      <template #body>
                         <div class="p-0 h-[calc(100vh-140px)] bg-gray-950 overflow-hidden flex flex-col">
                            <div class="flex-1 overflow-auto p-4 font-mono text-sm text-gray-300 whitespace-pre-wrap custom-scrollbar">
                               {{ reportContent }}
                            </div>
                         </div>
                      </template>
                  </UModal>
              </div>
           </template>

           <template #addons>
               <div class="h-full flex flex-col p-4 relative space-y-4">
                  <!-- Toolbar -->
                  <div class="flex items-center justify-between bg-gray-900/50 p-4 rounded-xl border border-gray-800 backdrop-blur-sm">
                     <div class="flex items-center gap-4">
                        <div class="p-3 bg-indigo-500/10 rounded-lg">
                           <UIcon :name="addonsFolder === 'mods' ? 'i-lucide-package' : 'i-lucide-puzzle'" class="w-6 h-6 text-indigo-400" />
                        </div>
                        <div>
                           <h3 class="font-bold text-lg text-white">Installed {{ addonsFolder === 'mods' ? 'Mods' : 'Plugins' }}</h3>
                           <p class="text-sm text-gray-400 flex items-center gap-2">
                              Manage your server extensions
                              <UBadge size="xs" color="neutral" variant="subtle">{{ addons.length }} installed</UBadge>
                           </p>
                        </div>
                     </div>
                     <div class="flex items-center gap-2">
                         <div class="flex bg-gray-800 p-1 rounded-lg border border-gray-700/50">
                           <UButton 
                              variant="ghost" 
                              size="xs"
                              color="neutral"
                              icon="i-lucide-grid-2x2" 
                              :class="{ 'bg-gray-700 text-white': viewMode === 'grid' }"
                              @click="viewMode = 'grid'" 
                           />
                           <UButton 
                              variant="ghost" 
                              size="xs"
                              color="neutral"
                              icon="i-lucide-list" 
                              :class="{ 'bg-gray-700 text-white': viewMode === 'list' }"
                              @click="viewMode = 'list'" 
                           />
                        </div>
                        <div class="h-8 w-px bg-gray-800 mx-2"></div>
                        <UButton 
                           icon="i-lucide-download" 
                           color="primary" 
                           size="md"
                           :label="addonsFolder === 'mods' ? 'Download Mods' : 'Download Plugins'" 
                           @click="showModrinthModal = true"
                           class="shadow-lg shadow-primary-500/20"
                        />
                     </div>
                  </div>

                  <!-- Content Area -->
                  <div class="flex-1 min-h-0 relative"> 
                     <div v-if="loadingAddons" class="absolute inset-0 flex flex-col items-center justify-center bg-gray-900/50 backdrop-blur-sm z-10 rounded-xl">
                         <UIcon name="i-lucide-loader-2" class="w-10 h-10 animate-spin text-primary-500 mb-4" />
                         <p class="text-gray-400 font-medium">Loading extensions...</p>
                     </div>
                     
                     <div v-else-if="addons.length === 0" class="flex flex-col items-center justify-center h-full text-gray-500 border-2 border-dashed border-gray-800 rounded-xl bg-gray-900/20">
                         <div class="p-4 bg-gray-800/50 rounded-full mb-4">
                           <UIcon name="i-lucide-package-open" class="w-10 h-10 opacity-50" />
                         </div>
                         <h4 class="text-lg font-bold text-gray-300">No extensions installed</h4>
                         <p class="text-sm mb-6">Start customization by downloading mods or plugins</p>
                         <UButton color="primary" variant="soft" @click="showModrinthModal = true">Browse Modrinth</UButton>
                     </div>

                     <div v-else class="h-full overflow-y-auto pr-2 custom-scrollbar">
                        <!-- Grid View -->
                        <div v-if="viewMode == 'grid'" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-4 pb-6">
                           <div 
                              v-for="mod in addons" 
                              :key="mod.fileName" 
                              class="group bg-gray-900/50 border border-gray-800 rounded-xl p-4 hover:border-primary-500/50 hover:bg-gray-800/80 transition-all duration-200 hover:shadow-lg hover:-translate-y-1 relative"
                           >
                              <div class="flex items-start gap-4 mb-3">
                                 <div class="relative flex-shrink-0">
                                    <img 
                                       v-if="mod.icon" 
                                       :src="mod.icon" 
                                       class="w-12 h-12 rounded-lg bg-gray-800 object-cover shadow-sm group-hover:scale-105 transition-transform" 
                                       alt="Icon"
                                       @error="(e) => (e.target as HTMLImageElement).style.display = 'none'"
                                    >
                                    <div v-else class="w-12 h-12 rounded-lg bg-gray-800 flex items-center justify-center border border-gray-700">
                                       <UIcon name="i-lucide-box" class="w-6 h-6 text-gray-500" />
                                    </div>
                                    <div class="absolute -bottom-1 -right-1 bg-gray-900 rounded-full p-0.5" v-if="mod.source === 'modrinth'">
                                       <UIcon name="i-simple-icons-modrinth" class="w-3 h-3 text-[#1bd96a]" />
                                    </div>
                                 </div>
                                 
                                 <div class="min-w-0 flex-1">
                                    <div class="flex items-start justify-between gap-2">
                                       <h4 class="font-bold text-white truncate text-base pt-0.5" :title="mod.title">{{ mod.title }}</h4>
                                    </div>
                                    <p class="text-xs text-gray-500 truncate font-mono mt-0.5">{{ mod.fileName }}</p>
                                 </div>
                              </div>

                              <div class="flex items-center justify-between mt-4 pt-4 border-t border-gray-800/50 group-hover:border-gray-700/50 transition-colors">
                                 <div class="flex gap-2">
                                    <UBadge v-if="mod.versionId" color="neutral" variant="subtle" size="xs">{{ mod.versionId?.slice(0, 8) }}</UBadge>
                                    <UBadge v-else color="neutral" variant="subtle" size="xs">Local File</UBadge>
                                 </div>
                                 
                                 <div class="flex items-center gap-1 opacity-60 group-hover:opacity-100 transition-opacity">
                                    <USwitch size="xs" :model-value="mod.enabled" @update:model-value="toggleAddon(mod)" color="success" />
                                    <div class="w-px h-4 bg-gray-700 mx-1"></div>
                                    <UTooltip v-if="mod.source === 'modrinth' && mod.latestVersionId" :text="`Update to ${mod.latestVersionNumber}`">
                                       <UButton icon="i-lucide-rotate-cw" color="primary" variant="ghost" size="xs" @click="updateAddon(mod)" />
                                    </UTooltip>
                                    <a v-if="mod.slug" :href="`https://modrinth.com/mod/${mod.slug}`" target="_blank" class="p-1 hover:bg-gray-700 rounded text-gray-400 hover:text-white transition-colors">
                                       <UIcon name="i-lucide-external-link" class="w-4 h-4" />
                                    </a>
                                    <UButton icon="i-lucide-trash-2" color="error" variant="ghost" size="xs" @click="deleteAddon(mod.fileName)" />
                                 </div>
                              </div>
                           </div>
                        </div>
                        
                        <!-- List View -->
                        <div v-else class="space-y-2 pb-6">
                           <div v-for="addon in addons" :key="addon.fileName" class="group bg-gray-900/30 border border-gray-800 hover:bg-gray-800/50 hover:border-gray-700 rounded-lg p-3 flex items-center gap-4 transition-all">
                                 <div class="flex-shrink-0 relative">
                                    <img v-if="addon.icon" :src="addon.icon" class="w-10 h-10 rounded-lg object-cover bg-gray-800" />
                                    <div v-else class="w-10 h-10 rounded-lg bg-gray-800 flex items-center justify-center">
                                       <UIcon name="i-lucide-box" class="w-5 h-5 text-gray-500" />
                                    </div>
                                 </div>

                                 <div class="flex-1 min-w-0 grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                                    <div>
                                       <div class="font-bold text-white truncate flex items-center gap-2">
                                          {{ addon.title || addon.fileName }}
                                          <UIcon v-if="addon.source === 'modrinth'" name="i-simple-icons-modrinth" class="w-3 h-3 text-[#1bd96a]" />
                                       </div>
                                       <div class="text-xs text-gray-500 truncate font-mono">{{ addon.fileName }}</div>
                                    </div>
                                    
                                    <div class="hidden md:flex gap-2">
                                       <UBadge v-if="addon.versionId" color="neutral" variant="subtle" size="xs">{{ addon.versionId }}</UBadge>
                                       <UBadge v-else color="neutral" variant="subtle" size="xs">Local Import</UBadge>
                                    </div>

                                    <div class="flex justify-end items-center gap-2 opacity-60 group-hover:opacity-100 transition-opacity">
                                       <USwitch 
                                          :model-value="addon.enabled" 
                                          @update:model-value="toggleAddon(addon)"
                                          color="success"
                                          size="md"
                                       />
                                       <div class="w-px h-5 bg-gray-700 mx-1"></div>
                                       <UTooltip v-if="addon.source === 'modrinth' && addon.latestVersionId" :text="`Update to ${addon.latestVersionNumber}`">
                                          <UButton icon="i-lucide-rotate-cw" color="primary" variant="ghost" size="xs" @click="updateAddon(addon)" />
                                       </UTooltip>
                                       <a v-if="addon.slug" :href="`https://modrinth.com/mod/${addon.slug}`" target="_blank">
                                          <UButton icon="i-lucide-external-link" color="neutral" variant="ghost" size="xs" />
                                       </a>
                                       <UButton icon="i-lucide-trash-2" color="error" variant="ghost" size="xs" @click="deleteAddon(addon.fileName)" />
                                    </div>
                                 </div>
                           </div>
                        </div>

                     </div>
                  </div>
                  
                  <!-- Modrinth Modal -->
                  <UModal fullscreen v-model:open="showModrinthModal" :ui="{ base: 'bg-gray-950', ring: 'ring-1 ring-gray-800' }">
                     <template #header>
                         <div class="flex items-center justify-between w-full gap-4 py-2">
                            <div class="flex items-center gap-3">
                               <div class="p-2 bg-[#1bd96a]/10 rounded-lg">
                                 <UIcon name="i-simple-icons-modrinth" class="w-6 h-6 text-[#1bd96a]" />
                               </div>
                               <div>
                                 <h2 class="text-lg font-bold text-white">Browse {{ addonsFolder === 'mods' ? 'Mods' : 'Plugins' }}</h2>
                                 <p class="text-xs text-gray-500">Powered by Modrinth</p>
                               </div>
                            </div>
                            
                            <div class="flex items-center gap-3 flex-1 max-w-xl bg-gray-900 border border-gray-800 p-1 rounded-xl">
                               <UInput 
                                  v-model="modrinthQuery" 
                                  icon="i-lucide-search" 
                                  placeholder="Search extensions..." 
                                  class="flex-1 border-none" 
                                  variant="none"
                                  :ui="{ icon: { leading: { pointer: '' } } }"
                                  @keydown.enter="searchModrinth"
                                  autofocus
                               />
                               <div class="w-px h-6 bg-gray-800"></div>
                               <USelectMenu
                                  v-model="modrinthSort"
                                  :items="sortOptions"
                                  value-key="value"
                                  label-key="label"
                                  class="w-40"
                                  variant="none"
                                  @update:model-value="searchModrinth"
                               />
                            </div>
                            
                            <UButton 
                               icon="i-lucide-x" 
                               color="neutral" 
                               variant="ghost" 
                               size="lg"
                               @click="showModrinthModal = false"
                               class="hover:bg-gray-800 rounded-xl"
                            />
                         </div>
                     </template>

                     <template #body>
                        <div class="flex h-full bg-gray-950">
                           <!-- Categories Sidebar -->
                           <div class="w-64 border-r border-gray-800/50 bg-gray-900/20 overflow-y-auto flex-shrink-0 p-4 space-y-2 custom-scrollbar">
                              <div class="mb-4">
                                 <p class="text-xs font-bold text-gray-500 uppercase tracking-widest mb-2 px-2">Filters</p>
                                 <UButton 
                                    color="neutral" 
                                    variant="soft" 
                                    size="sm" 
                                    icon="i-lucide-filter-x"
                                    class="w-full justify-start"
                                    :disabled="!selectedCategory"
                                    @click="selectedCategory = ''; searchModrinth()"
                                 >
                                    Reset Filters
                                 </UButton>
                              </div>
                              
                              <div>
                                 <p class="text-xs font-bold text-gray-500 uppercase tracking-widest mb-2 px-2">Categories</p>
                                 <div class="space-y-1">
                                    <button
                                       v-for="cat in filteredCategories"
                                       :key="cat.name"
                                       class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-left transition-all border border-transparent"
                                       :class="selectedCategory === cat.name 
                                          ? 'bg-primary-500/10 text-primary-400 border-primary-500/20 shadow-sm' 
                                          : 'hover:bg-gray-800 text-gray-400 hover:text-gray-200'"
                                       @click="selectedCategory = cat.name; searchModrinth()"
                                    >
                                       <span v-html="cat.icon" class="w-4 h-4 flex-shrink-0 opacity-80"></span>
                                       <span class="capitalize truncate flex-1">{{ cat.name.replace(/-/g, ' ') }}</span>
                                       <UIcon v-if="selectedCategory === cat.name" name="i-lucide-check" class="w-3 h-3" />
                                    </button>
                                 </div>
                              </div>
                           </div>
                           
                           <!-- Results Grid -->
                           <div class="flex-1 overflow-y-auto p-6 bg-dots-dark">
                              <div v-if="searchingModrinth" class="flex flex-col items-center justify-center h-[50vh]">
                                 <UIcon name="i-lucide-loader-2" class="w-12 h-12 animate-spin text-primary-500 mb-4" />
                                 <p class="text-gray-400 animate-pulse">Searching Modrinth library...</p>
                              </div>
                              
                              <div v-else-if="modrinthResults.length === 0" class="flex flex-col items-center justify-center h-[50vh] text-center text-gray-500">
                                 <div class="p-6 bg-gray-900 rounded-full border border-gray-800 mb-4 shadow-xl">
                                    <UIcon name="i-lucide-search" class="w-12 h-12 opacity-30" />
                                 </div>
                                 <h3 class="text-xl font-bold text-gray-300 mb-2">No results found</h3>
                                 <p class="max-w-md mx-auto">Try customizing your search terms or selecting a different category to browse.</p>
                              </div>
                              
                              <div v-else class="grid grid-cols-1 xl:grid-cols-2 gap-4 max-w-7xl mx-auto">
                                 <div 
                                    v-for="hit in modrinthResults" 
                                    :key="hit.slug" 
                                    class="flex gap-4 p-4 rounded-xl bg-gray-900 border border-gray-800 hover:border-primary-500/30 hover:bg-gray-800/80 transition-all hover:shadow-lg group"
                                 >
                                    <img 
                                       :src="hit.icon_url || 'https://cdn.modrinth.com/placeholder.svg'" 
                                       class="w-20 h-20 rounded-xl bg-gray-800 object-cover flex-shrink-0 shadow-md group-hover:scale-105 transition-transform duration-300" 
                                    />
                                    <div class="flex-1 min-w-0 flex flex-col">
                                       <div class="flex items-start justify-between gap-2 mb-1">
                                          <div>
                                             <h4 class="font-bold text-lg text-white truncate group-hover:text-primary-400 transition-colors">{{ hit.title }}</h4>
                                             <div class="text-xs text-gray-400 flex items-center gap-1.5">
                                                <span>by {{ hit.author }}</span>
                                                <span class="w-1 h-1 bg-gray-600 rounded-full"></span>
                                                <span class="flex items-center gap-0.5"><UIcon name="i-lucide-download" class="w-3 h-3"/> {{ formatNumber(hit.downloads) }}</span>
                                             </div>
                                          </div>
                                          <UBadge v-if="installedSlugs.has(hit.slug)" size="xs" color="success" variant="soft" class="shadow-sm shadow-success-500/10">
                                             <UIcon name="i-lucide-check" class="w-3 h-3 mr-1" />
                                             Installed
                                          </UBadge>
                                       </div>
                                       
                                       <p class="text-sm text-gray-400 line-clamp-2 mb-3 flex-1 leading-relaxed">{{ hit.description }}</p>
                                       
                                       <div class="flex items-center justify-between mt-auto pt-3 border-t border-gray-800/50">
                                          <div class="flex gap-1.5 flex-wrap">
                                             <span v-for="cat in (hit.categories || []).slice(0, 3)" :key="cat" class="px-1.5 py-0.5 rounded text-[10px] uppercase font-bold bg-gray-800 text-gray-400 border border-gray-700/50">
                                                {{ cat }}
                                             </span>
                                          </div>
                                          <UButton 
                                             v-if="!installedSlugs.has(hit.slug)"
                                             size="sm" 
                                             color="primary" 
                                             variant="solid" 
                                             icon="i-lucide-download" 
                                             :loading="installingSlug === hit.slug"
                                             :disabled="installingSlug !== null"
                                             @click="installFromModrinth(hit)"
                                             class="flex-shrink-0 shadow-lg shadow-primary-500/20 opacity-0 group-hover:opacity-100 transition-opacity"
                                          >
                                             Install
                                          </UButton>
                                       </div>
                                    </div>
                                 </div>
                              </div>
                           </div>
                        </div>
                     </template>
                  </UModal>
               </div>
            </template>

           <!-- Player Management -->
           <template #players>
               <div class="h-full overflow-y-auto p-4 custom-scrollbar">
                  <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
                     <!-- Online Players -->
                     <div class="space-y-4">
                        <div class="flex items-center justify-between">
                           <div class="flex items-center gap-2">
                              <UIcon name="i-lucide-users" class="w-5 h-5 text-indigo-400" />
                              <h3 class="font-bold text-lg text-white">Online Players</h3>
                           </div>
                           <UBadge :color="serverStatus === 'online' ? 'success' : 'neutral'" variant="subtle" size="md">
                              {{ onlinePlayers.length }} Online
                           </UBadge>
                        </div>
                        
                        <UCard class="h-[400px] flex flex-col">
                           <template #header>
                             <div class="flex justify-between items-center py-1">
                                <span class="text-xs font-bold text-gray-500 uppercase tracking-wider">Player List</span>
                                <UButton icon="i-lucide-refresh-cw" color="neutral" variant="ghost" size="xs" @click="fetchOnlinePlayers" :loading="loadingPlayers" />
                             </div>
                           </template>

                           <div class="flex-1 overflow-y-auto min-h-0 -mx-4 px-4 space-y-1 custom-scrollbar">
                              <div v-if="serverStatus !== 'online'" class="h-full flex flex-col items-center justify-center text-gray-500">
                                 <div class="p-4 bg-gray-900/50 rounded-full mb-3">
                                    <UIcon name="i-lucide-wifi-off" class="w-8 h-8 opacity-40" />
                                 </div>
                                 <p class="font-medium">Server is offline</p>
                              </div>
                              <div v-else-if="onlinePlayers.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
                                 <div class="p-4 bg-gray-900/50 rounded-full mb-3">
                                   <UIcon name="i-lucide-user" class="w-8 h-8 opacity-40" />
                                 </div>
                                 <p class="font-medium">No players online</p>
                              </div>
                              <div v-else v-for="player in onlinePlayers" :key="player" class="flex items-center justify-between p-3 bg-gray-900/40 rounded-xl border border-transparent hover:border-gray-700/50 hover:bg-gray-800/60 transition-all group">
                                 <div class="flex items-center gap-3">
                                    <img :src="`https://mc-heads.net/avatar/${player}/40`" class="w-10 h-10 rounded-lg bg-gray-800 shadow-sm" :alt="player" />
                                    <span class="font-bold text-gray-200">{{ player }}</span>
                                 </div>
                                 <div class="flex items-center gap-1 opacity-60 group-hover:opacity-100 transition-opacity">
                                    <UTooltip text="Quick Actions">
                                       <UDropdownMenu  :items="[
                                          [{ label: 'Operator', icon: 'i-lucide-star', click: () => quickOp(player) }],
                                          [{ label: 'Kick', icon: 'i-lucide-log-out', click: () => kickPlayer(player) }, { label: 'Ban', icon: 'i-lucide-ban', color: 'error', click: () => quickBan(player) }]
                                       ]">
                                          <UButton icon="i-lucide-more-horizontal" color="neutral" variant="ghost" size="xs" />
                                       </UDropdownMenu >
                                    </UTooltip>
                                 </div>
                              </div>
                           </div>
                        </UCard>
                     </div>

                     <!-- Whitelist -->
                     <div class="space-y-4">
                        <div class="flex items-center justify-between">
                           <div class="flex items-center gap-2">
                              <UIcon name="i-lucide-shield-check" class="w-5 h-5 text-emerald-400" />
                              <h3 class="font-bold text-lg text-white">Whitelist</h3>
                           </div>
                           <UBadge color="neutral" variant="subtle" size="md">{{ whitelist.length }} Allowed</UBadge>
                        </div>
                        
                         <UCard class="h-[400px] flex flex-col">
                           <template #header>
                             <div class="flex items-center gap-2 py-1">
                                <UInput 
                                   v-model="newWhitelistPlayer" 
                                   placeholder="Add player..." 
                                   icon="i-lucide-user-plus"
                                   size="sm" 
                                   class="flex-1" 
                                   :ui="{ icon: { leading: { pointer: '' } } }"
                                   @keydown.enter="addToWhitelist" 
                                />
                                <UButton icon="i-lucide-plus" color="emerald" variant="solid" size="sm" @click="addToWhitelist" :disabled="!newWhitelistPlayer" />
                             </div>
                           </template>

                           <div class="flex-1 overflow-y-auto min-h-0 -mx-4 px-4 space-y-1 custom-scrollbar">
                              <div v-if="whitelist.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
                                 <UIcon name="i-lucide-list-x" class="w-10 h-10 mb-2 opacity-30" />
                                 <p>Whitelist is empty</p>
                              </div>
                              <div v-else v-for="entry in whitelist" :key="entry.uuid" class="flex items-center justify-between p-3 bg-gray-900/40 rounded-xl border border-transparent hover:border-gray-700/50 hover:bg-gray-800/60 transition-all group">
                                 <div class="flex items-center gap-3">
                                    <img :src="`https://mc-heads.net/avatar/${entry.name}/40`" class="w-10 h-10 rounded-lg bg-gray-800 shadow-sm" :alt="entry.name" />
                                    <div class="flex flex-col">
                                       <span class="font-bold text-gray-200">{{ entry.name }}</span>
                                       <span class="text-[10px] text-gray-500 font-mono">{{ entry.uuid.substring(0, 8) }}...</span>
                                    </div>
                                 </div>
                                 <UButton icon="i-lucide-trash-2" color="error" variant="ghost" size="xs" class="opacity-0 group-hover:opacity-100 transition-opacity" @click="removeFromWhitelist(entry.uuid)" />
                              </div>
                           </div>
                        </UCard>
                     </div>

                     <!-- Operators -->
                     <div class="space-y-4">
                        <div class="flex items-center justify-between">
                           <div class="flex items-center gap-2">
                              <UIcon name="i-lucide-crown" class="w-5 h-5 text-amber-400" />
                              <h3 class="font-bold text-lg text-white">Operators</h3>
                           </div>
                           <UBadge color="neutral" variant="subtle" size="md">{{ operators.length }} Admins</UBadge>
                        </div>
                        
                        <UCard class="h-[400px] flex flex-col">
                           <template #header>
                             <div class="flex items-center gap-2 py-1">
                                <UInput 
                                   v-model="newOperator" 
                                   placeholder="Add operator..." 
                                   icon="i-lucide-shield-alert"
                                   size="sm" 
                                   class="flex-1" 
                                   :ui="{ icon: { leading: { pointer: '' } } }"
                                   @keydown.enter="addOperator" 
                                />
                                <UButton icon="i-lucide-plus" color="amber" variant="solid" size="sm" @click="addOperator" :disabled="!newOperator" />
                             </div>
                           </template>
                           
                           <div class="flex-1 overflow-y-auto min-h-0 -mx-4 px-4 space-y-1 custom-scrollbar">
                              <div v-if="operators.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
                                 <UIcon name="i-lucide-shield-off" class="w-10 h-10 mb-2 opacity-30" />
                                 <p>No operators assigned</p>
                              </div>
                              <div v-else v-for="entry in operators" :key="entry.uuid" class="flex items-center justify-between p-3 bg-gray-900/40 rounded-xl border border-transparent hover:border-gray-700/50 hover:bg-gray-800/60 transition-all group">
                                 <div class="flex items-center gap-3">
                                    <img :src="`https://mc-heads.net/avatar/${entry.name}/40`" class="w-10 h-10 rounded-lg bg-gray-800 shadow-sm" :alt="entry.name" />
                                    <div class="flex flex-col">
                                       <span class="font-bold text-gray-200">{{ entry.name }}</span>
                                       <UBadge color="amber" variant="subtle" size="xs" class="w-fit">Level {{ entry.level }}</UBadge>
                                    </div>
                                 </div>
                                 <UButton icon="i-lucide-trash-2" color="error" variant="ghost" size="xs" class="opacity-0 group-hover:opacity-100 transition-opacity" @click="removeOperator(entry.uuid)" />
                              </div>
                           </div>
                        </UCard>
                     </div>

                     <!-- Bans -->
                     <div class="space-y-4">
                        <div class="flex items-center justify-between">
                           <div class="flex items-center gap-2">
                              <UIcon name="i-lucide-gavel" class="w-5 h-5 text-rose-400" />
                              <h3 class="font-bold text-lg text-white">Bans</h3>
                           </div>
                           <UBadge color="neutral" variant="subtle" size="md">{{ bannedPlayers.length }} Banned</UBadge>
                        </div>
                        
                        <UCard class="h-[400px] flex flex-col">
                           <template #header>
                             <div class="flex items-center gap-2 py-1">
                                <UInput 
                                   v-model="newBannedPlayer" 
                                   placeholder="Player..." 
                                   size="sm" 
                                   class="flex-1" 
                                   @keydown.enter="banPlayer" 
                                />
                                <UInput 
                                   v-model="banReason" 
                                   placeholder="Reason"
                                   size="sm" 
                                   class="flex-1" 
                                   @keydown.enter="banPlayer" 
                                />
                                <UButton icon="i-lucide-gavel" color="rose" variant="solid" size="sm" @click="banPlayer" :disabled="!newBannedPlayer" />
                             </div>
                           </template>
                           
                           <div class="flex-1 overflow-y-auto min-h-0 -mx-4 px-4 space-y-1 custom-scrollbar">
                              <div v-if="bannedPlayers.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
                                 <UIcon name="i-lucide-check-circle-2" class="w-10 h-10 mb-2 opacity-30" />
                                 <p>No banned players</p>
                              </div>
                              <div v-else v-for="entry in bannedPlayers" :key="entry.uuid" class="flex items-center justify-between p-3 bg-gray-900/40 rounded-xl border border-transparent hover:border-gray-700/50 hover:bg-gray-800/60 transition-all group">
                                 <div class="flex items-center gap-3">
                                    <img :src="`https://mc-heads.net/avatar/${entry.name}/40`" class="w-10 h-10 rounded-lg bg-gray-800 shadow-sm" :alt="entry.name" />
                                    <div class="flex flex-col min-w-0">
                                       <span class="font-bold text-gray-200">{{ entry.name }}</span>
                                       <span class="text-xs text-rose-400 truncate max-w-[150px]" :title="entry.reason">{{ entry.reason || 'No reason' }}</span>
                                    </div>
                                 </div>
                                 <UButton icon="i-lucide-rotate-ccw" color="success" variant="ghost" size="xs" class="opacity-0 group-hover:opacity-100 transition-opacity" @click="unbanPlayer(entry.uuid)" tooltip="Unban" />
                              </div>
                           </div>
                        </UCard>
                     </div>
                  </div>
               </div>
            </template>
        </UTabs>
      </div>
      <!-- EULA Modal -->
      <UModal v-model:open="showEulaModal">
         <template #header>
            <div class="flex items-center gap-2 text-warning-500">
               <UIcon name="i-lucide-scroll-text" class="w-6 h-6" />
               <h3 class="font-bold text-lg text-white">Accept EULA</h3>
            </div>
         </template>
         
         <template #body>
            <div class="space-y-4">
               <p class="text-sm text-gray-300">
                  By using this software, you agree to the <a href="https://aka.ms/MinecraftEULA" target="_blank" class="text-primary-500 hover:underline">Minecraft End User License Agreement</a>.
               </p>
               <p class="text-xs text-gray-500">
                  You must accept the EULA to start the server.
               </p>
            </div>
         </template>


         <template #footer>
            <div class="flex justify-end gap-2">
               <UButton color="neutral" variant="ghost" @click="showEulaModal = false">Cancel</UButton>
               <UButton color="primary" @click="acceptEula">I Agree</UButton>
            </div>
         </template>
      </UModal>

      <!-- Properties Editor Modal -->
      <UModal v-model:open="showPropertiesEditor">
         <template #header>
            <div class="flex items-center gap-2">
               <UIcon name="i-lucide-file-code" class="w-5 h-5 text-primary-500" />
               <h3 class="font-bold text-lg text-white">Edit server.properties</h3>
            </div>
         </template>
         
         <template #body>
            <div class="space-y-4">
               <p class="text-xs text-gray-500">Edit the raw server.properties file directly. Be careful with formatting.</p>
               <UTextarea 
                  v-model="propertiesEditorContent" 
                  :rows="20" 
                  class="font-mono w-full text-xs"
                  placeholder="# server.properties content..."
               />
            </div>
         </template>

         <template #footer>
            <div class="flex justify-end gap-2">
               <UButton color="neutral" variant="ghost" @click="showPropertiesEditor = false">Cancel</UButton>
               <UButton color="primary" icon="i-lucide-save" @click="savePropertiesFromEditor">Save Changes</UButton>
            </div>
         </template>
      </UModal>
    </main>
  </div>
</template>

<script setup lang="ts">
import { readTextFile, writeTextFile, readDir, remove, exists, mkdir, writeFile, rename, copyFile, readFile, BaseDirectory } from '@tauri-apps/plugin-fs'
import { fetch } from '@tauri-apps/plugin-http'
import { Command, type Child, open } from '@tauri-apps/plugin-shell'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { join } from '@tauri-apps/api/path'
import { documentDir } from '@tauri-apps/api/path'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { parseAnsiToHtml } from '~/utils/ansiParser'
import { installModpack, installMrpack } from '~/utils/modpack'

const route = useRoute()
const router = useRouter()
const toast = useToast()
const serverProcessStore = useServerProcessStore()
const serversStore = useServersStore()
const tunnelStore = useTunnelStore()

let viewMode = ref<'grid' | 'list'>('grid')




const serverId = route.params.id as string // actually folder name

const serverFolderName = computed(() => route.params.id as string)

const loading = ref(true)
const saving = ref(false)
const server = ref<any>(null)
const serverIconUrl = ref<string | null>(null)

function updateServerIconUrl(blob: Blob) {
    if (serverIconUrl.value) {
        URL.revokeObjectURL(serverIconUrl.value)
    }
    serverIconUrl.value = URL.createObjectURL(blob)
}

onUnmounted(() => {
    if (serverIconUrl.value) {
        URL.revokeObjectURL(serverIconUrl.value)
    }
})

// Settings
const serverName = ref('')
const javaSettings = reactive({
   memory: 4,
   path: 'java',
   flags: ''
})

// System RAM
const systemRamGB = ref(32) // Default fallback

// Properties
const rawProperties = ref('')
const parsedProperties = ref<Record<string, any>>({})
const propsSearch = ref('')

// Properties Editor Modal
const showPropertiesEditor = ref(false)
const propertiesEditorContent = ref('')

watch(showPropertiesEditor, (isOpen) => {
   if (isOpen) {
      // Load raw content when modal opens
      propertiesEditorContent.value = rawProperties.value
   }
})

// MOTD Editor
const motdTextarea = ref<HTMLTextAreaElement | null>(null)

const mcColorCodes = [
   { code: '0', color: '#000000', textColor: '#fff', name: 'Black' },
   { code: '1', color: '#0000AA', textColor: '#fff', name: 'Dark Blue' },
   { code: '2', color: '#00AA00', textColor: '#fff', name: 'Dark Green' },
   { code: '3', color: '#00AAAA', textColor: '#000', name: 'Dark Aqua' },
   { code: '4', color: '#AA0000', textColor: '#fff', name: 'Dark Red' },
   { code: '5', color: '#AA00AA', textColor: '#fff', name: 'Dark Purple' },
   { code: '6', color: '#FFAA00', textColor: '#000', name: 'Gold' },
   { code: '7', color: '#AAAAAA', textColor: '#000', name: 'Gray' },
   { code: '8', color: '#555555', textColor: '#fff', name: 'Dark Gray' },
   { code: '9', color: '#5555FF', textColor: '#fff', name: 'Blue' },
   { code: 'a', color: '#55FF55', textColor: '#000', name: 'Green' },
   { code: 'b', color: '#55FFFF', textColor: '#000', name: 'Aqua' },
   { code: 'c', color: '#FF5555', textColor: '#000', name: 'Red' },
   { code: 'd', color: '#FF55FF', textColor: '#000', name: 'Light Purple' },
   { code: 'e', color: '#FFFF55', textColor: '#000', name: 'Yellow' },
   { code: 'f', color: '#FFFFFF', textColor: '#000', name: 'White' },
]

const mcStyleCodes = [
   { code: 'l', label: 'B', class: 'font-bold', name: 'Bold' },
   { code: 'o', label: 'I', class: 'italic', name: 'Italic' },
   { code: 'n', label: 'U', class: 'underline', name: 'Underline' },
   { code: 'm', label: 'S', class: 'line-through', name: 'Strikethrough' },
   { code: 'k', label: '?', class: '', name: 'Obfuscated' },
   { code: 'r', label: 'R', class: 'text-gray-400', name: 'Reset' },
]

// Modpack Update State
const showUpdateModal = ref(false)
const checkingUpdate = ref(false)
const isUpdating = ref(false)
const updateAvailable = ref(false)
const updateData = ref<any>(null)

const mcColorMap: Record<string, string> = {
   '0': '#000000', '1': '#0000AA', '2': '#00AA00', '3': '#00AAAA',
   '4': '#AA0000', '5': '#AA00AA', '6': '#FFAA00', '7': '#AAAAAA',
   '8': '#555555', '9': '#5555FF', 'a': '#55FF55', 'b': '#55FFFF',
   'c': '#FF5555', 'd': '#FF55FF', 'e': '#FFFF55', 'f': '#FFFFFF',
}

// Console log parsing
interface ParsedLogLine {
   time: string
   level: string
   message: string
}

function parseLogLine(line: string): ParsedLogLine | null {
   // 1. Clean line for pattern matching (remove ANSI and extra whitespace)
   // eslint-disable-next-line no-control-regex
   const cleanLine = line.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '').trim()

   // Pattern 1: [HH:MM:SS LEVEL]: message
   const pattern1 = /^\[(\d{2}:\d{2}:\d{2})\s+(INFO|WARN|ERROR|DEBUG|FATAL)\]:\s*(.*)$/i
   let match = cleanLine.match(pattern1)
   
   if (match) {
      const time = match[1]
      const level = match[2].toUpperCase()
      // Extract message from original line to preserve colors
      // Find the first occurrence of "]:" which likely marks end of header
      const splitIdx = line.indexOf(']:')
      const message = splitIdx !== -1 ? line.substring(splitIdx + 2).replace(/^\s+/, '') : match[3]
      return { time, level, message }
   }
   
   // Pattern 2: [HH:MM:SS] [Thread/LEVEL]: message
   const pattern2 = /^\[(\d{2}:\d{2}:\d{2})\]\s+\[.*?\/(INFO|WARN|ERROR|DEBUG|FATAL)\]:\s*(.*)$/i
   match = cleanLine.match(pattern2)
   
   if (match) {
      const time = match[1]
      const level = match[2].toUpperCase()
      const splitIdx = line.indexOf(']:')
      const message = splitIdx !== -1 ? line.substring(splitIdx + 2).replace(/^\s+/, '') : match[3]
      return { time, level, message }
   }

   // Pattern 3: [HH:MM:SS] [Thread/LEVEL] [Context]: message
   const pattern3 = /^\[(\d{2}:\d{2}:\d{2})\]\s+\[.*?\/(INFO|WARN|ERROR|DEBUG|FATAL)\].*?:\s*(.*)$/i
   match = cleanLine.match(pattern3)

   if (match) {
      const time = match[1]
      const level = match[2].toUpperCase()
      const splitIdx = line.indexOf(']:') // Assumes standard formatting
      const message = splitIdx !== -1 ? line.substring(splitIdx + 2).replace(/^\s+/, '') : match[3]
      return { time, level, message }
   }
   
   // Pattern 4: [LEVEL] message
   const pattern4 = /^\[(INFO|WARN|ERROR|DEBUG|FATAL)\]\s*(.*)$/i
   match = cleanLine.match(pattern4)
   
   if (match) {
      const level = match[1].toUpperCase()
      const splitIdx = line.indexOf(']')
      const message = splitIdx !== -1 ? line.substring(splitIdx + 1).replace(/^\s+/, '') : match[2]
      return { time: '', level, message }
   }
   
   return null
}

function getLogLevelColor(level: string): 'primary' | 'success' | 'warning' | 'error' | 'neutral' {
   switch (level) {
      case 'INFO': return 'primary'
      case 'WARN': return 'warning'
      case 'ERROR': return 'error'
      case 'FATAL': return 'error'
      case 'DEBUG': return 'neutral'
      default: return 'neutral'
   }
}

function scrollToBottom() {
   const el = consoleRef.value
   if (el) {
      el.scrollTop = el.scrollHeight
   }
}

function insertMotdCode(code: string) {
   const textarea = motdTextarea.value
   if (!textarea) return
   
   const start = textarea.selectionStart
   const end = textarea.selectionEnd
   const currentValue = parsedProperties.value['motd'] || ''
   const newValue = currentValue.slice(0, start) + '§' + code + currentValue.slice(end)
   parsedProperties.value['motd'] = newValue
   
   // Restore cursor position
   nextTick(() => {
      textarea.focus()
      textarea.selectionStart = textarea.selectionEnd = start + 2
   })
}

function renderMotdPreview(motd: string): string {
   if (!motd) return '<span class="text-gray-500">A Minecraft Server</span>'
   
   let result = ''
   let currentColor = '#AAAAAA'
   let bold = false, italic = false, underline = false, strike = false
   
   let i = 0
   while (i < motd.length) {
      if (motd[i] === '§' && i + 1 < motd.length) {
         const code = motd[i + 1].toLowerCase()
         
         if (mcColorMap[code]) {
            currentColor = mcColorMap[code]
            bold = italic = underline = strike = false
         } else if (code === 'l') bold = true
         else if (code === 'o') italic = true
         else if (code === 'n') underline = true
         else if (code === 'm') strike = true
         else if (code === 'r') {
            currentColor = '#AAAAAA'
            bold = italic = underline = strike = false
         }
         
         i += 2
         continue
      }
      
      let styles = `color: ${currentColor};`
      if (bold) styles += ' font-weight: bold;'
      if (italic) styles += ' font-style: italic;'
      if (underline) styles += ' text-decoration: underline;'
      if (strike) styles += ' text-decoration: line-through;'
      
      const char = motd[i] === '<' ? '&lt;' : motd[i] === '>' ? '&gt;' : motd[i]
      result += `<span style="${styles}">${char}</span>`
      i++
   }
   
   return result || '<span class="text-gray-500">A Minecraft Server</span>'
}

// --- Addons (Mods/Plugins) Logic ---
// --- Addons (Mods/Plugins) Logic ---
interface AddonMeta {
    title: string
    icon?: string
    slug?: string
    versionId?: string
    fileName: string
    source: 'modrinth' | 'local'
}

interface AddonUI extends AddonMeta {
    latestVersionId?: string
    latestVersionNumber?: string
    enabled: boolean
}

const addons = ref<AddonUI[]>([]) // Combined list
const installedAddonsMeta = ref<Record<string, Omit<AddonMeta, 'fileName' | 'source'>>>({}) // Persistent meta
const loadingAddons = ref(false)
const checkingUpdates = ref(false)
const installingSlug = ref<string | null>(null) // Currently installing addon slug

// Get set of installed slugs for quick lookup
const installedSlugs = computed(() => {
   const slugs = new Set<string>()
   Object.values(installedAddonsMeta.value).forEach(meta => {
      if (meta.slug) slugs.add(meta.slug)
   })
   return slugs
})

// --- Player Management ---
interface WhitelistEntry {
   uuid: string
   name: string
}

interface OperatorEntry {
   uuid: string
   name: string
   level: number
   bypassesPlayerLimit: boolean
}

interface BannedPlayerEntry {
   uuid: string
   name: string
   created: string
   source: string
   expires: string
   reason: string
}

const onlinePlayers = ref<string[]>([])
const whitelist = ref<WhitelistEntry[]>([])
const operators = ref<OperatorEntry[]>([])
const bannedPlayers = ref<BannedPlayerEntry[]>([])
const loadingPlayers = ref(false)

// Track processed lines separately from consoleLines because we filter some out!
const lastLogLineCount = ref(0)

// Poll for online players (RCON fallback support)
async function fetchOnlinePlayers() {
   if (serverStatus.value !== 'online') {
       onlinePlayers.value = []
       // Reset log pointer when offline so next start works cleanly
       lastLogLineCount.value = 0
       return
   }

   // Fallback to RCON if no process or if specifically checking
   // We prefer RCON for parsing cleanly if available
   const rconPort = getPropertyValue('rcon.port') || '25575'
   const rconPassword = getPropertyValue('rcon.password') || 'voidlink'
   
   if (!serverProcess.value || getPropertyValue('enable-rcon') === 'true') {
      try {
          const res = await invoke<{success: boolean, response: string}>('rcon_send_command', {
              host: '127.0.0.1', 
              port: parseInt(rconPort),
              password: rconPassword,
              command: 'list'
          })
          
          if (res.success) {
              // Parse: "There are 1 of 20 players online: VoIdLiNk"
              // or "There are 0 of 20 players online: "
              const match = res.response.match(/online:(.*)/)
              if (match) {
                  const names = match[1].split(',').map(n => n.trim()).filter(n => n)
                  onlinePlayers.value = names
              }
          }
      } catch(e) {
          // RCON failed, ignore
      }
   }
   
   // If we have process, also send list to console just in case (optional, but keep console alive)
   if (serverProcess.value) {
       // We don't need to spam 'list' in console if we use RCON for the UI list.
   }
}

// Log Tailing for Live Console (when process handle lost)
async function tailLogs() {
    if (serverStatus.value !== 'online' || serverProcess.value) {
       // We only tail if online AND no process handle
       // If we have process handle, we get logs via stdout event
       // If offline, we reset pointer via fetchOnlinePlayers or here
       return
    }
    
    try {
        const folder = serverFolderName.value
        // Must use absolute path for Rust file reading if possible, or relative to app
        // But our utils.rs expects a path. Let's resolve the path fully.
        // We can construct it via documentDir
        const docs = await documentDir()
        const logPath = await join(docs, 'VoidLink', 'servers', folder, 'logs', 'latest.log')
        
        const result = await invoke<{content: string, new_offset: number}>('read_log_tail', {
            path: logPath,
            offset: lastLogLineCount.value // REUSED VARIABLE NAME: Using 'lastLogLineCount' as 'lastByteOffset' to avoid big refactor
        })
        
        const newContent = result.content
        if (!newContent) {
           // No new content, check if we need to reset due to file shrink happens inside Rust?
           // Rust command returns new_offset=0 if file shrank.
        }
        
        // If offset reset to 0 (file shrank/rotated)
        if (result.new_offset < lastLogLineCount.value) {
             consoleLines.value.length = 0
             lastLogLineCount.value = 0
             // But we might have content from the new file start
        }

        if (newContent) {
            const lines = newContent.split('\n')
            for (const l of lines) {
               // Filter RCON spam
               if (l.trim() && !l.includes('Thread RCON Client /127.0.0.1')) {
                  consoleLines.value.push(l)
               }
            }
        }
        
        // Update offset
        lastLogLineCount.value = result.new_offset
        
    } catch (e) {
        // failed to read log, maybe locked or file not found
    }
}

let playerPollInterval: any = null
let logTailInterval: any = null

onMounted(() => {
    playerPollInterval = setInterval(fetchOnlinePlayers, 120000)
    logTailInterval = setInterval(tailLogs, 2000) // Poll logs every 2s
    fetchOnlinePlayers()
})

onUnmounted(() => {
    if (playerPollInterval) clearInterval(playerPollInterval)
    if (logTailInterval) clearInterval(logTailInterval)
})

// Input fields for adding new entries
const newWhitelistPlayer = ref('')
const newOperator = ref('')
const newBannedPlayer = ref('')
const banReason = ref('')

// ...

async function toggleAddon(addon: AddonUI) {
   const folder = serverFolderName.value
   const oldName = addon.fileName
   // If currently enabled (ends with .jar), disable it (append .disabled)
   // If currently disabled (.disabled), enable it (remove .disabled)
   // But easier: check addon.enabled flag derived from extension.
   
   const newName = addon.enabled ? `${oldName}.disabled` : oldName.replace(/\.disabled$/, '')
   
   const oldPath = `VoidLink/servers/${folder}/${addonsFolder.value}/${oldName}`
   const newPath = `VoidLink/servers/${folder}/${addonsFolder.value}/${newName}`
   
   try {
       await rename(oldPath, newPath, { oldPathBaseDir: BaseDirectory.Document, newPathBaseDir: BaseDirectory.Document })
       
       // Update Meta Key
       if (installedAddonsMeta.value[oldName]) {
           installedAddonsMeta.value[newName] = installedAddonsMeta.value[oldName]
           delete installedAddonsMeta.value[oldName]
           await saveAddonsMeta()
       }
       
       loadAddons()
   } catch (e) {
       console.error('Toggle failed', e)
   }
}

// ...



async function checkForUpdates() {
    checkingUpdates.value = true
    try {
        const serverVersion = server.value.version
        const serverType = server.value.type

        let loaders: string[] = []
        if (serverType === 'fabric') loaders = ['fabric']
        else if (serverType === 'forge') loaders = ['forge']
        else if (serverType === 'neoforge') loaders = ['neoforge']
        else if (serverType === 'paper' || serverType === 'purpur') loaders = ['paper', 'bukkit', 'spigot']
        else if (serverType === 'velocity') loaders = ['velocity']
        
        const params = new URLSearchParams()
        if (loaders.length) params.append('loaders', JSON.stringify(loaders))
        if (serverVersion) params.append('game_versions', JSON.stringify([serverVersion]))

        // Check for each addon in parallel
        await Promise.all(addons.value.map(async (addon) => {
            if (addon.source !== 'modrinth' || !addon.slug) return
            
            try {
                const res = await fetch(`https://api.modrinth.com/v2/project/${addon.slug}/version?${params.toString()}`)
                const versions = await res.json()
                
                if (versions && versions.length > 0) {
                    const validVersion = versions[0]
                    
                    if (validVersion.id !== addon.versionId) {
                        let isActuallyCurrent = false
                        
                        // Check if current file matches the remote version's files (Auto-Heal)
                        const remoteFiles = validVersion.files || []
                        const localFileName = addon.fileName.replace('.disabled', '')
                        const match = remoteFiles.find((f: any) => f.filename === localFileName)
                        
                        if (match) {
                            isActuallyCurrent = true
                            // Update local meta with missing version ID
                            if (installedAddonsMeta.value[addon.fileName]) {
                                installedAddonsMeta.value[addon.fileName].versionId = validVersion.id
                                saveAddonsMeta()
                            }
                        }

                        if (!isActuallyCurrent) {
                            addon.latestVersionId = validVersion.id
                            addon.latestVersionNumber = validVersion.version_number
                        }
                    }
                }
            } catch (ignore) { /* verify failed for specific addon */ }
        }))

    } catch (e) {
        console.error('Update check failed', e)
    } finally {
        checkingUpdates.value = false
    }
}

async function updateAddon(addon: AddonUI) {
    if (!addon.latestVersionId || !addon.slug) return
    
    // 1. Delete old
    await deleteAddon(addon.fileName)
    
    // 2. Install new (Simulate project object)
    // We need to fetch versions AGAIN or pass the cached ID?
    // Let's modify installFromModrinth or just duplicate logic slightly for stability.
    // Better: Re-use installFromModrinth logic but with just slug/meta.
    const projectStub = { title: addon.title, slug: addon.slug, icon_url: addon.icon }
    await installFromModrinth(projectStub)
}

// ...

// Modrinth browser state
const showModrinthBrowser = ref(true)

// Format large numbers nicely (e.g. 1000000 -> 1M)
function formatNumber(num: number): string {
   if (num >= 1000000) return (num / 1000000).toFixed(1).replace(/\.0$/, '') + 'M'
   if (num >= 1000) return (num / 1000).toFixed(1).replace(/\.0$/, '') + 'K'
   return num.toString()
}

// Update installFromModrinth to save versionId


const modrinthQuery = ref('')
const modrinthResults = ref<any[]>([])
const searchingModrinth = ref(false)
const showModrinthModal = ref(false)
const selectedCategory = ref('')
const modrinthSort = ref('relevance')
const modrinthCategories = ref<{ name: string; icon: string; project_type: string }[]>([])
const loadingCategories = ref(false)

const sortOptions = [
   { label: 'Relevance', value: 'relevance' },
   { label: 'Downloads', value: 'downloads' },
   { label: 'Follows', value: 'follows' },
   { label: 'Newest', value: 'newest' },
   { label: 'Updated', value: 'updated' }
]

// Filtered categories for current project type (mod or plugin)
const filteredCategories = computed(() => {
   // Modrinth uses 'mod' for both mods and plugins
   // Also include categories with no specific project_type
   return modrinthCategories.value.filter(c => 
      c.project_type === 'mod' || c.project_type === 'project' || !c.project_type
   )
})

async function loadModrinthCategories() {
   if (modrinthCategories.value.length > 0) {
      return
   }
   loadingCategories.value = true
   try {
      const res = await fetch('https://api.modrinth.com/v2/tag/category')
      const data = await res.json()
      modrinthCategories.value = data
   } catch (e) {
      console.error('[Categories] Failed to load:', e)
   } finally {
      loadingCategories.value = false
   }
}

// Load categories and initial results when modal opens
watch(showModrinthModal, (isOpen) => {
   if (isOpen) {
      loadModrinthCategories()
      searchModrinth()
   }
})



// Use composable for server state persistence
const serverStore = useServerProcessStore()
const storeServerId = computed(() => serverFolderName.value)
const serverState = computed(() => serverStore.getServer(storeServerId.value))

import { useJava } from '~/composables/useJava'
const { scanJava, getJavaForVersion, validateJavaPath, installations } = useJava()

// These are reactive references to the store - use store methods for persistence
const serverStatus = computed({
   get: () => serverState.value.status,
   set: (val) => { serverStore.setStatus(storeServerId.value, val) }
})
const serverProcess = computed({
   get: () => serverState.value.process,
   set: (val) => { serverStore.setProcess(storeServerId.value, val) }
})

// Direct reference to console lines array - mutations like .push() work
const consoleLines = computed(() => serverState.value.consoleLines)
const consoleRef = ref<HTMLElement | null>(null)
const consoleInput = ref('')

// Send command to server - uses stdin if available, fallback to RCON
async function sendCommand() {
   const cmd = consoleInput.value.trim()
   if (!cmd) return
   
   consoleLines.value.push(`> ${cmd}`)
   consoleInput.value = ''
   
   // Try using Child stdin first
   if (serverProcess.value) {
      try {
         await serverProcess.value.write(cmd + '\n')
         return
      } catch (e) {
         console.error('Failed to write to stdin:', e)
      }
   }
   
   // Fallback to RCON
   const rconPort = getPropertyValue('rcon.port') || '25575'
   const rconPassword = getPropertyValue('rcon.password') || 'voidlink'
   
   try {
      const result = await invoke<{ success: boolean; response: string }>('rcon_send_command', {
         host: '127.0.0.1',
         port: parseInt(rconPort),
         password: rconPassword,
         command: cmd
      })
      if (result.response) {
         consoleLines.value.push(result.response)
      }
   } catch (e) {
      consoleLines.value.push(`[RCON Error] ${e}`)
   }
}

// Load logs from file (for recovered servers after F5)
async function loadLogsFromFile() {
   try {
      const folder = serverFolderName.value
      const logPath = `VoidLink/servers/${folder}/logs/latest.log`
      
      if (!await exists(logPath, { baseDir: BaseDirectory.Document })) {
         return
      }
      
      const content = await readTextFile(logPath, { baseDir: BaseDirectory.Document })
      const lines = content.split('\n').slice(-200) // Last 200 lines
      
      // Clear and add log lines
      serverState.value.consoleLines.length = 0
      for (const line of lines) {
         if (line.trim()) {
            serverState.value.consoleLines.push(line)
         }
      }
   } catch (e) {
      console.error('Failed to load logs from file:', e)
   }
}

// Auto-load logs if server is recovered (no Child handle but online)
watch(() => serverStatus.value, async (status) => {
   if (status === 'online' && !serverProcess.value && consoleLines.value.length === 0) {
      await loadLogsFromFile()
   }
}, { immediate: true })

watch(() => consoleLines.value.length, async () => {
   await nextTick()
   if (consoleRef.value) {
       consoleRef.value.scrollTop = consoleRef.value.scrollHeight
   }
})
const showEulaModal = ref(false)

const statusColor = computed(() => {
   switch(serverStatus.value) {
      case 'online': return 'success'
      case 'starting': return 'warning'
      case 'stopping': return 'warning'
      default: return 'neutral'
   }
})

const addonsFolder = computed(() => {
   if (!server.value) return null
   const t = server.value.type
   if (['fabric', 'forge', 'neoforge', 'quilt'].includes(t)) return 'mods'
   if (['paper', 'purpur', 'folia', 'velocity'].includes(t)) return 'plugins'
   if (t === 'modpack') return 'mods' // Modpacks typically use mods
   return null
})

async function saveAddonsMeta() {
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/addons.json`
      await writeTextFile(path, JSON.stringify(installedAddonsMeta.value, null, 2), { baseDir: BaseDirectory.Document })
   } catch (e) {
      console.error('Failed to save addons meta', e)
   }
}

async function loadAddons() {
   if (!addonsFolder.value) return
   loadingAddons.value = true
   addons.value = []
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/${addonsFolder.value}`
      const metaPath = `VoidLink/servers/${folder}/addons.json`
      
      // Load Meta
      if (await exists(metaPath, { baseDir: BaseDirectory.Document })) {
         const content = await readTextFile(metaPath, { baseDir: BaseDirectory.Document })
         installedAddonsMeta.value = JSON.parse(content)
      } else {
         installedAddonsMeta.value = {}
      }

      if (!(await exists(path, { baseDir: BaseDirectory.Document }))) {
         await mkdir(path, { baseDir: BaseDirectory.Document, recursive: true })
      }
      
      const entries = await readDir(path, { baseDir: BaseDirectory.Document })
      
      addons.value = entries
         .filter(e => e.isFile && (e.name.endsWith('.jar') || e.name.endsWith('.jar.disabled')))
         .map(e => {
             const enabled = !e.name.endsWith('.disabled')
             const meta = installedAddonsMeta.value[e.name]
             if (meta) {
                 return {
                     fileName: e.name,
                     title: meta.title,
                     icon: meta.icon,
                     slug: meta.slug,
                     versionId: meta.versionId,
                     source: 'modrinth',
                     enabled
                 }
             }
             return {
                 fileName: e.name,
                 title: e.name.replace('.jar.disabled', '.jar'),
                 source: 'local',
                 enabled
             }
         })
         
      checkForUpdates()
   } catch (e) {
      console.error('Failed to load addons', e)
   } finally {
      loadingAddons.value = false
   }
}

async function handleDrop(e: DragEvent) {
   if (!e.dataTransfer?.files) return
   
   const files = Array.from(e.dataTransfer.files)
   for (const file of files) {
      if (file.name.endsWith('.jar')) {
         const buffer = await file.arrayBuffer()
         const targetPath = `VoidLink/servers/${serverFolderName.value}/${addonsFolder.value}/${file.name}`
         await writeFile(targetPath, new Uint8Array(buffer), { baseDir: BaseDirectory.Document })
      }
   }
   loadAddons()
}

watch(addonsFolder, (newVal) => {
   if (newVal) loadAddons()
})

async function searchModrinth() {
   // Allow empty query if category is selected (browsing)
   if (!modrinthQuery.value && !selectedCategory.value) {
       // If mostly empty, maybe show popular? For now just return or specific behavior.
       // actually Modrinth allows empty query with facets.
   }

   searchingModrinth.value = true
   modrinthResults.value = []
   try {
      const serverVersion = server.value.version
      const serverType = server.value.type
      
      // Loader mapping
      let loaders: string[] = []
      if (['fabric', 'quilt'].includes(serverType)) loaders = ['fabric', 'quilt']
      else if (['forge'].includes(serverType)) loaders = ['forge']
      else if (['neoforge'].includes(serverType)) loaders = ['neoforge']
      else if (['velocity'].includes(serverType)) loaders = ['velocity']
      else if (['paper', 'purpur', 'spigot'].includes(serverType)) loaders = ['paper', 'purpur', 'spigot', 'bukkit']

      // Construct facets
      // [[version], [loader]] -> AND logic
      const facets = []
      
      // Version constraint
      if (serverVersion) facets.push(`["versions:${serverVersion}"]`)
      
      // Loader constraint (OR logic within)
      if (loaders.length > 0) {
          const loaderFacets = loaders.map(l => `"categories:${l}"`).join(',')
          facets.push(`[${loaderFacets}]`) 
      }
      
      // Project type constraint (mod or plugin)
      const typeFacet = addonsFolder.value === 'mods' ? '"project_type:mod"' : '"project_type:plugin"'
      facets.push(`[${typeFacet}]`)

      // User selected category filter
      if (selectedCategory.value) {
          facets.push(`["categories:${selectedCategory.value}"]`)
      }

      const facetsStr = `[${facets.join(',')}]`
      
      const res = await fetch(`https://api.modrinth.com/v2/search?query=${encodeURIComponent(modrinthQuery.value)}&facets=${facetsStr}&index=${modrinthSort.value}&limit=30`)
      const data = await res.json()
      modrinthResults.value = data.hits || []
   } catch (e) {
      console.error('Modrinth search failed', e)
   } finally {
      searchingModrinth.value = false
   }
}

async function installFromModrinth(project: any) {
   installingSlug.value = project.slug
   try {
       const serverVersion = server.value.version
       const serverType = server.value.type

       // Map server type to modrinth loader format for /version endpoint
       let loaders: string[] = []
       if (serverType === 'fabric') loaders = ['fabric']
       else if (serverType === 'forge') loaders = ['forge']
       else if (serverType === 'neoforge') loaders = ['neoforge']
       else if (serverType === 'paper' || serverType === 'purpur') loaders = ['paper', 'bukkit', 'spigot']
       else if (serverType === 'velocity') loaders = ['velocity']
       
       const params = new URLSearchParams()
       if (loaders.length) params.append('loaders', JSON.stringify(loaders))
       if (serverVersion) params.append('game_versions', JSON.stringify([serverVersion]))

       const res = await fetch(`https://api.modrinth.com/v2/project/${project.slug}/version?${params.toString()}`)
       const versions = await res.json()
       
       if (!versions || versions.length === 0) {
          console.warn('No compatible version found')
          return
       }
       
       const validVersion = versions[0]
       const file = validVersion.files.find((f: any) => f.primary) || validVersion.files[0]
       
       if (file) {
          const downloadUrl = file.url
          const fileName = file.filename
          
          await downloadAddon(downloadUrl, fileName)
          
          // Save Metadata
          installedAddonsMeta.value[fileName] = {
              title: project.title,
              icon: project.icon_url,
              slug: project.slug
          }
          
          // Download required dependencies
          const dependencies = validVersion.dependencies || []
          const requiredDeps = dependencies.filter((d: any) => d.dependency_type === 'required')
          
          for (const dep of requiredDeps) {
             // Skip if already installed
             if (dep.project_id && installedSlugs.value.has(dep.project_id)) continue
             
             try {
                // Get dependency project info
                const projectRes = await fetch(`https://api.modrinth.com/v2/project/${dep.project_id}`)
                const depProject = await projectRes.json()
                
                // Skip if already installed by slug
                if (depProject.slug && installedSlugs.value.has(depProject.slug)) continue
                
                // Get version (use specific version_id if provided, otherwise get latest compatible)
                let depVersion
                if (dep.version_id) {
                   const versionRes = await fetch(`https://api.modrinth.com/v2/version/${dep.version_id}`)
                   depVersion = await versionRes.json()
                } else {
                   const depVersionsRes = await fetch(`https://api.modrinth.com/v2/project/${dep.project_id}/version?${params.toString()}`)
                   const depVersions = await depVersionsRes.json()
                   if (depVersions && depVersions.length > 0) {
                      depVersion = depVersions[0]
                   }
                }
                
                if (depVersion) {
                   const depFile = depVersion.files.find((f: any) => f.primary) || depVersion.files[0]
                   if (depFile) {
                      await downloadAddon(depFile.url, depFile.filename)
                      
                      // Save dependency metadata
                      installedAddonsMeta.value[depFile.filename] = {
                         title: depProject.title,
                         icon: depProject.icon_url,
                         slug: depProject.slug
                      }
                   }
                }
             } catch (depError) {
                console.warn('Failed to install dependency:', dep.project_id, depError)
             }
          }
          
          await saveAddonsMeta()
          await loadAddons()
       }
   } catch (e) {
      console.error('Install failed', e)
   } finally {
      installingSlug.value = null
   }
}

async function downloadAddon(url: string, fileName: string) {
   const folder = serverFolderName.value
   const targetPath = `VoidLink/servers/${folder}/${addonsFolder.value}/${fileName}`
   
   const res = await fetch(url)
   const buffer = await res.arrayBuffer()
   
   await writeFile(targetPath, new Uint8Array(buffer), { baseDir: BaseDirectory.Document })
   // Note: loadAddons is called by caller usually, or we can leave it here if we want immediate feedback for simple downloads.
   // But to avoid double refresh with meta, caller handles it.
}

async function deleteAddon(fileName: string) {
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/${addonsFolder.value}/${fileName}`
      await remove(path, { baseDir: BaseDirectory.Document })
      
      if (installedAddonsMeta.value[fileName]) {
          delete installedAddonsMeta.value[fileName]
          await saveAddonsMeta()
      }
      
      loadAddons()
   } catch (e) { console.error('Delete failed', e) }
}

// Watch for tab change to load addons
watch(() => server.value, () => {
   // if tab is addons? 
})
// Actually we can just load on mount if specific tab active or lazy load.

// --- Player Management Logic ---

async function loadPlayerLists() {
   loadingPlayers.value = true
   try {
      const folder = serverFolderName.value
      
      // Load whitelist.json
      try {
         const whitelistPath = `VoidLink/servers/${folder}/whitelist.json`
         if (await exists(whitelistPath, { baseDir: BaseDirectory.Document })) {
            const content = await readTextFile(whitelistPath, { baseDir: BaseDirectory.Document })
            whitelist.value = JSON.parse(content)
         } else {
            whitelist.value = []
         }
      } catch (e) {
         console.error('Failed to load whitelist', e)
         whitelist.value = []
      }
      
      // Load ops.json
      try {
         const opsPath = `VoidLink/servers/${folder}/ops.json`
         if (await exists(opsPath, { baseDir: BaseDirectory.Document })) {
            const content = await readTextFile(opsPath, { baseDir: BaseDirectory.Document })
            operators.value = JSON.parse(content)
         } else {
            operators.value = []
         }
      } catch (e) {
         console.error('Failed to load operators', e)
         operators.value = []
      }
      
      // Load banned-players.json
      try {
         const bannedPath = `VoidLink/servers/${folder}/banned-players.json`
         if (await exists(bannedPath, { baseDir: BaseDirectory.Document })) {
            const content = await readTextFile(bannedPath, { baseDir: BaseDirectory.Document })
            bannedPlayers.value = JSON.parse(content)
         } else {
            bannedPlayers.value = []
         }
      } catch (e) {
         console.error('Failed to load banned players', e)
         bannedPlayers.value = []
      }
      
   } catch (e) {
      console.error('Failed to load player lists', e)
   } finally {
      loadingPlayers.value = false
   }
}

async function saveWhitelist() {
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/whitelist.json`
      await writeTextFile(path, JSON.stringify(whitelist.value, null, 2), { baseDir: BaseDirectory.Document })
   } catch (e) {
      console.error('Failed to save whitelist', e)
   }
}

async function saveOperators() {
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/ops.json`
      await writeTextFile(path, JSON.stringify(operators.value, null, 2), { baseDir: BaseDirectory.Document })
   } catch (e) {
      console.error('Failed to save operators', e)
   }
}

async function saveBannedPlayers() {
   try {
      const folder = serverFolderName.value
      const path = `VoidLink/servers/${folder}/banned-players.json`
      await writeTextFile(path, JSON.stringify(bannedPlayers.value, null, 2), { baseDir: BaseDirectory.Document })
   } catch (e) {
      console.error('Failed to save banned players', e)
   }
}

async function fetchPlayerUUID(playerName: string): Promise<{ uuid: string; name: string } | null> {
   try {
      const res = await fetch(`https://api.mojang.com/users/profiles/minecraft/${playerName}`)
      if (!res.ok) return null
      const data = await res.json()
      const id = data.id
      const uuid = `${id.slice(0,8)}-${id.slice(8,12)}-${id.slice(12,16)}-${id.slice(16,20)}-${id.slice(20)}`
      return { uuid, name: data.name }
   } catch (e) {
      console.error('Failed to fetch UUID', e)
      return null
   }
}

async function addToWhitelist() {
   if (!newWhitelistPlayer.value.trim()) return
   
   const playerData = await fetchPlayerUUID(newWhitelistPlayer.value.trim())
   if (!playerData) return
   
   if (whitelist.value.some(e => e.uuid === playerData.uuid)) {
      newWhitelistPlayer.value = ''
      return
   }
   
   whitelist.value.push({ uuid: playerData.uuid, name: playerData.name })
   await saveWhitelist()
   newWhitelistPlayer.value = ''
   
   if (serverStatus.value === 'online' && serverProcess.value) {
      await serverProcess.value.write(`whitelist add ${playerData.name}\n`)
   }
}

async function removeFromWhitelist(uuid: string) {
   const entry = whitelist.value.find(e => e.uuid === uuid)
   whitelist.value = whitelist.value.filter(e => e.uuid !== uuid)
   await saveWhitelist()
   
   if (serverStatus.value === 'online' && serverProcess.value && entry) {
      await serverProcess.value.write(`whitelist remove ${entry.name}\n`)
   }
}

async function addOperator() {
   if (!newOperator.value.trim()) return
   
   const playerData = await fetchPlayerUUID(newOperator.value.trim())
   if (!playerData) return
   
   if (operators.value.some(e => e.uuid === playerData.uuid)) {
      newOperator.value = ''
      return
   }
   
   operators.value.push({ uuid: playerData.uuid, name: playerData.name, level: 4, bypassesPlayerLimit: false })
   await saveOperators()
   newOperator.value = ''
   
   if (serverStatus.value === 'online' && serverProcess.value) {
      await serverProcess.value.write(`op ${playerData.name}\n`)
   }
}

async function removeOperator(uuid: string) {
   const entry = operators.value.find(e => e.uuid === uuid)
   operators.value = operators.value.filter(e => e.uuid !== uuid)
   await saveOperators()
   
   if (serverStatus.value === 'online' && serverProcess.value && entry) {
      await serverProcess.value.write(`deop ${entry.name}\n`)
   }
}

async function banPlayer() {
   if (!newBannedPlayer.value.trim()) return
   
   const playerData = await fetchPlayerUUID(newBannedPlayer.value.trim())
   if (!playerData) return
   
   if (bannedPlayers.value.some(e => e.uuid === playerData.uuid)) {
      newBannedPlayer.value = ''
      banReason.value = ''
      return
   }
   
   bannedPlayers.value.push({ 
      uuid: playerData.uuid, 
      name: playerData.name,
      created: new Date().toISOString(),
      source: 'VoidLink',
      expires: 'forever',
      reason: banReason.value || 'Banned by administrator'
   })
   await saveBannedPlayers()
   
   if (serverStatus.value === 'online' && serverProcess.value) {
      await serverProcess.value.write(`ban ${playerData.name} ${banReason.value || 'Banned by administrator'}\n`)
   }
   
   newBannedPlayer.value = ''
   banReason.value = ''
}

async function unbanPlayer(uuid: string) {
   const entry = bannedPlayers.value.find(e => e.uuid === uuid)
   bannedPlayers.value = bannedPlayers.value.filter(e => e.uuid !== uuid)
   await saveBannedPlayers()
   
   if (serverStatus.value === 'online' && serverProcess.value && entry) {
      await serverProcess.value.write(`pardon ${entry.name}\n`)
   }
}

// Quick actions for online players
async function kickPlayer(playerName: string) {
   if (serverStatus.value !== 'online' || !serverProcess.value) return
   await serverProcess.value.write(`kick ${playerName}\n`)
   // Refresh player list after kick
   setTimeout(() => fetchOnlinePlayers(), 500)
}

async function quickBan(playerName: string) {
   if (serverStatus.value !== 'online' || !serverProcess.value) return
   await serverProcess.value.write(`ban ${playerName} Banned via VoidLink\n`)
   // Refresh lists
   setTimeout(() => {
      fetchOnlinePlayers()
      loadPlayerLists()
   }, 500)
}

async function quickOp(playerName: string) {
   if (serverStatus.value !== 'online' || !serverProcess.value) return
   await serverProcess.value.write(`op ${playerName}\n`)
   // Refresh operators list
   setTimeout(() => loadPlayerLists(), 500)
}

async function quickWhitelist(playerName: string) {
   if (serverStatus.value !== 'online' || !serverProcess.value) return
         await serverProcess.value.write(`whitelist add ${playerName}\n`)
   // Refresh whitelist
   setTimeout(() => loadPlayerLists(), 500)
}

async function openServerFolder() {
    try {
        const folder = serverFolderName.value
        const relative = `VoidLink/servers/${folder}`
        const docDir = await documentDir()
        const fullPath = await join(docDir, relative)
        
        // Detect platform
        const isWindows = navigator.userAgent.includes('Windows')
        
        if (isWindows) {
            const command = Command.create('run-bat', ['/C', 'start .'], { cwd: fullPath })
            await command.spawn()
        } else {
            // macOS/Linux
            const command = Command.create('run-sh', ['-c', `open "${fullPath}"`])
            await command.spawn()
        }
    } catch (e) {
        console.error('Failed to open folder', e)
        consoleLines.value.push(`Failed to open folder: ${e}`)
    }
}



const tabs = computed(() => {
    const allTabs = [
        { label: 'Performance', icon: 'i-lucide-activity', value: 'performance', slot: 'performance' as const },
        { label: 'Console', icon: 'i-lucide-terminal', value: 'console', slot: 'console' as const },
        { label: 'Settings', icon: 'i-lucide-settings', value: 'settings', slot: 'settings' as const },
        { label: 'Mods/Plugins', icon: 'i-lucide-package', value: 'addons', slot: 'addons' as const },
        { label: 'Players', icon: 'i-lucide-users', value: 'players', slot: 'players' as const },
        { label: 'Crash Reports', icon: 'i-lucide-file-warning', value: 'crash-reports', slot: 'crash-reports' as const }
    ]
    
    if (!addonsFolder.value) {
        return allTabs.filter(t => t.value !== 'mods')
    }
    
    return allTabs
})

const selectedTab = ref('performance')

watch(selectedTab, async () => {
   const tab = selectedTab.value
   if (!tab) return
   
   // Scroll console to bottom when switching to console tab
   if (tab === 'console') {
      await nextTick()
      if (consoleRef.value) {
         consoleRef.value.scrollTop = consoleRef.value.scrollHeight
      }
   }
   
   if (tab === 'players' && whitelist.value.length === 0 && operators.value.length === 0 && bannedPlayers.value.length === 0) {
      loadPlayerLists()
      fetchOnlinePlayers()
   }
   if (tab === 'mods' && !Object.keys(addons.value).length) {
      loadAddons()
   }
   if (tab === 'crash-reports') {
       loadCrashReports()
   }
})


// --- Initialization ---

let statsInterval: any | null = null

onMounted(async () => {
   selectedTab.value = 'performance' // Ensure Console is selected by default
   await loadData()
   loadAddons() // Load addons after server data is loaded
   
   // Start stats polling
   statsInterval = setInterval(() => {
       if (serverStatus.value === 'online' || serverStatus.value === 'starting') {
           serverProcessStore.refreshProcessInfo(serverId as string)
       }
   }, 2000)
})

onUnmounted(() => {
    if (statsInterval) clearInterval(statsInterval)
})

async function changeServerIcon() {
    try {
        const selected = await openDialog({
            multiple: false,
            filters: [{
                name: 'Image',
                extensions: ['png', 'jpg', 'jpeg']
            }]
        })

        if (Array.isArray(selected) || !selected) return

        const folder = serverFolderName.value
        // Destination: VoidLink/servers/<id>/server-logo.png
        const iconRelPath = `VoidLink/servers/${folder}/server-logo.png`
        
        await copyFile(selected, iconRelPath, { toPathBaseDir: BaseDirectory.Document })
        
        // Update state
        const bytes = await readFile(iconRelPath, { baseDir: BaseDirectory.Document })
        const blob = new Blob([bytes], { type: 'image/png' })
        updateServerIconUrl(blob)
        
        console.log('Icon updated')
    } catch (e) {
        console.error('Failed to change icon', e)
        toast.add({ title: 'Failed to update icon', description: String(e), color: 'error' })
    }
}

async function loadData() {
   loading.value = true
   try {
      const folder = serverFolderName.value // e.g., "myserver-xyz789"
      
      // 1. Load server.json
      try {
         // Using plain path string construction because join from fs plugin was deprecated/moved
         const metaPath = `VoidLink/servers/${folder}/server.json`
         const metaContent = await readTextFile(metaPath, { baseDir: BaseDirectory.Document })
         server.value = JSON.parse(metaContent)
         serverName.value = server.value.name || ''

         // Load global settings for defaults
         let globalSettings = { memory: 4, path: 'java', flags: '' }
         try {
            const globalContent = await readTextFile('VoidLink/settings.json', { baseDir: BaseDirectory.Document })
            const gData = JSON.parse(globalContent)
            if (gData) {
               globalSettings = { ...globalSettings, ...gData, memory: gData.defaultMemory, flags: gData.defaultFlags }
            }
         } catch(e) { /* ignore */ }

         // Load saved settings if exist, else use globals
         if (server.value.javaSettings) {
             javaSettings.memory = server.value.javaSettings.memory ?? globalSettings.memory
             javaSettings.path = server.value.javaSettings.path ?? ( globalSettings.path || 'java' )
             javaSettings.flags = server.value.javaSettings.flags ?? globalSettings.flags
         } else {
             javaSettings.memory = globalSettings.memory
             javaSettings.path = globalSettings.path || 'java'
             javaSettings.flags = globalSettings.flags
         }
      } catch (e) {
         console.error('Failed to load server.json', e)
      }

      // 2. Load server.properties
      try {
         const propsPath = `VoidLink/servers/${folder}/server.properties`
         const propsContent = await readTextFile(propsPath, { baseDir: BaseDirectory.Document })
         rawProperties.value = propsContent
         parsedProperties.value = parseProperties(propsContent)
      } catch (e) {
         console.log('No server.properties found (maybe not started yet?)')
         // Populate with defaults if empty so user can edit before start
         parsedProperties.value = getDefaultProperties()
      }
      
      // 3. Get system RAM for slider max
      try {
         const sysInfo = await invoke<{ total_memory_bytes: number }>('get_system_info')
         systemRamGB.value = Math.floor(sysInfo.total_memory_bytes / (1024 * 1024 * 1024))
      } catch (e) {
         console.log('Failed to check sys info', e)
      }

      // 4. Load Custom Icon
      try {
          const iconPath = `VoidLink/servers/${folder}/server-logo.png`
          if (await exists(iconPath, { baseDir: BaseDirectory.Document })) {
              const bytes = await readFile(iconPath, { baseDir: BaseDirectory.Document })
              const blob = new Blob([bytes], { type: 'image/png' })
              updateServerIconUrl(blob)
          } else {
             if (serverIconUrl.value) URL.revokeObjectURL(serverIconUrl.value)
             serverIconUrl.value = null
          }
      } catch (e) {
          console.error('Failed to load server icon', e)
      }

   } catch (e) {
      console.error('Error loading server data', e)
   } finally {
      loading.value = false
   }
}

// --- Properties Logic ---

function parseProperties(content: string) {
   const props: Record<string, any> = {}
   content.split('\n').forEach(line => {
      line = line.trim()
      if (!line || line.startsWith('#')) return
      const [key, ...rest] = line.split('=')
      if (key) {
         props[key.trim()] = rest.join('=').trim()
      }
   })
   return props
}

function getDefaultProperties() {
   // Minimal set to ensure UI shows something useful
   return {
      'server-port': '25565',
      'gamemode': 'survival',
      'difficulty': 'easy',
      'max-players': '20',
      'online-mode': 'true',
      'motd': 'A Minecraft Server',
      'enable-rcon': 'true',
      'rcon.port': '25575',
      'broadcast-rcon-to-ops': 'false',
      'rcon.password': 'voidlink'
   }
}

// Property Definitions for UI Grouping
const propertyDefinitions = {
   'gamemode': { type: 'select', options: ['survival', 'creative', 'adventure', 'spectator'], group: 'Gameplay' },
   'difficulty': { type: 'select', options: ['peaceful', 'easy', 'normal', 'hard'], group: 'Gameplay' },
   'pvp': { type: 'boolean', group: 'Gameplay' },
   'hardcore': { type: 'boolean', group: 'Gameplay' },
   'enable-command-block': { type: 'boolean', group: 'Gameplay' },
   'allow-flight': { type: 'boolean', group: 'Gameplay' },
   
   'server-port': { type: 'number', group: 'Network' },
   'max-players': { type: 'number', group: 'Network' },
   'online-mode': { type: 'boolean', group: 'Network' },
   'enable-rcon': { type: 'boolean', group: 'Network' },
   'rcon.port': { type: 'number', group: 'Network' },
   'rcon.password': { type: 'text', group: 'Network' },
   'view-distance': { type: 'number', group: 'Performance' },
   'simulation-distance': { type: 'number', group: 'Performance' },
   
   'motd': { type: 'text', group: 'General' },
   'level-name': { type: 'text', group: 'General' },
   'level-seed': { type: 'text', group: 'General' },
}

// Computed for Search & Grouping
const groupedProperties = computed(() => {
   const query = propsSearch.value.toLowerCase()
   const groups: Record<string, any[]> = {}
   
   // Default group
   groups['Advanced'] = []
   
   Object.entries(parsedProperties.value).forEach(([key, val]) => {
      if (query && !key.toLowerCase().includes(query)) return

      const def = propertyDefinitions[key as keyof typeof propertyDefinitions]
      const type = def?.type || inferType(val)
      const group = def?.group || 'Advanced'
      
      // Transform value for UI (bool string -> bool)
      let uiValue: any = val
      if (type === 'boolean') uiValue = val === 'true'
      
      const propObj = {
         key,
         value: uiValue, // We need to sync this back to string on save
         type,
         options: def?.options
      }
      
      if (!groups[group]) groups[group] = []
      groups[group].push(propObj)
   })
   
   // Remove empty groups
   Object.keys(groups).forEach(k => {
      if (groups[k].length === 0) delete groups[k]
   })
   
   return groups
})

const accordionItems = computed(() => {
   // Order: General, Gameplay, Network, Performance, Advanced
   const order = ['General', 'Gameplay', 'Network', 'Performance', 'Advanced']
   const existingGroups = Object.keys(groupedProperties.value)
   const sorted = existingGroups.sort((a, b) => {
      const ia = order.indexOf(a)
      const ib = order.indexOf(b)
      if (ia === -1) return 1
      if (ib === -1) return -1
      return ia - ib
   })
   
   return sorted.map(group => ({
      label: group,
      defaultOpen: group !== 'Advanced',
      properties: groupedProperties.value[group]
   }))
})

function inferType(val: string) {
   if (val === 'true' || val === 'false') return 'boolean'
   if (!isNaN(Number(val)) && val.trim() !== '') return 'number'
   return 'text'
}

function formatPropLabel(key: string) {
   return key.replace(/-/g, ' ').replace(/\b\w/g, l => l.toUpperCase())
}

function getPropertyValue(key: string): string {
   return parsedProperties.value[key] || ''
}

function updateProperty(key: string, value: any) {
   // Convert boolean back to string 'true'/'false'
   // Keep numbers/strings as is
   let strVal = String(value)
   parsedProperties.value[key] = strVal
}

function updateAccessProperty(key: string, value: boolean) {
   updateProperty(key, value)
   if (key === 'white-list' && value === false) {
      updateProperty('enforce-whitelist', false)
   }
}

// --- Saving ---

async function saveAllSettings() {
   saving.value = true
   try {
      const folder = serverFolderName.value
      
      // 1. Save server.json (Name & Java Settings)
      server.value.javaSettings = { ...javaSettings }
      server.value.name = serverName.value
      const metaPath = `VoidLink/servers/${folder}/server.json`
      await writeTextFile(metaPath, JSON.stringify(server.value, null, 2), { baseDir: BaseDirectory.Document })

      // 2. Save server.properties
      let content = '#Minecraft server properties\n#Generated by VoidLink\n'
      Object.entries(parsedProperties.value).forEach(([key, val]) => {
         content += `${key}=${val}\n`
      })
      const propsPath = `VoidLink/servers/${folder}/server.properties`
      await writeTextFile(propsPath, content, { baseDir: BaseDirectory.Document })
      
      // Refresh raw properties view
      rawProperties.value = content

      // 3. Refresh global server list (Pinia) - NOW calls after properties are saved so it reads new port
      const serversStore = useServersStore()
      await serversStore.refreshServers()

      // Show toast (mock)
      console.log('All settings saved')
   } catch (e) {
      console.error('Failed to save settings', e)
   } finally {
      saving.value = false
   }
}

async function savePropertiesFromEditor() {
   try {
      const folder = serverFolderName.value
      const propsPath = `VoidLink/servers/${folder}/server.properties`
      
      // Save raw content
      await writeTextFile(propsPath, propertiesEditorContent.value, { baseDir: BaseDirectory.Document })
      
      // Update local state
      rawProperties.value = propertiesEditorContent.value
      
      // Re-parse properties
      const lines = propertiesEditorContent.value.split('\n')
      const parsed: Record<string, string> = {}
      for (const line of lines) {
         if (line.startsWith('#') || !line.includes('=')) continue
         const [key, ...rest] = line.split('=')
         parsed[key.trim()] = rest.join('=').trim()
      }
      parsedProperties.value = parsed
      
      // Close modal
      showPropertiesEditor.value = false
      
      console.log('Properties saved from editor')
   } catch (e) {
      console.error('Failed to save properties from editor', e)
   }
}


// --- Modpack Updates ---

async function checkModpackUpdate() {
   if (!server.value?.modpack?.id || server.value.modpack.id === 'custom') return

   checkingUpdate.value = true
   try {
      const res = await fetch(`https://api.modrinth.com/v2/project/${server.value.modpack.id}/version`)
      const versions = await res.json()
      
      if (versions.length > 0) {
         const latest = versions.find((v: any) => v.version_type === 'release') || versions[0]
         
         // Check if version is different from current
         // Assuming server.value.modpack.versionId matches modrinth version ID
         if (latest.id !== server.value.modpack.versionId) {
            updateAvailable.value = true
            updateData.value = latest
            showUpdateModal.value = true
         } else {
            console.log('Modpack is up to date')
            // Optional: toast logic here
         }
      }
   } catch (e) {
      console.error('Failed to check updates', e)
   } finally {
      checkingUpdate.value = false
   }
}

async function performModpackUpdate() {
   if (!updateData.value) return
   
   isUpdating.value = true
   try {
      // 1. Stop server if running
      if (serverStatus.value !== 'offline' && serverProcess.value) {
         await stopServer()
         // Wait for stop
         let retries = 0
         while (serverStatus.value !== 'offline' && retries < 20) {
            await new Promise(r => setTimeout(r, 500))
            retries++
         }
      }

      const folder = serverFolderName.value
      const relativePath = `VoidLink/servers/${folder}`
      const fullServerPath = await join(await documentDir(), relativePath)

      // 2. Clean mods and config folders
      // Note: This is destructive but standard for modpack updates
      const modsPath = await join(fullServerPath, 'mods')
      if (await exists(modsPath, { baseDir: BaseDirectory.Document })) {
          await remove(modsPath, { baseDir: BaseDirectory.Document, recursive: true })
      }
      
      const configPath = await join(fullServerPath, 'config')
       if (await exists(configPath, { baseDir: BaseDirectory.Document })) {
          await remove(configPath, { baseDir: BaseDirectory.Document, recursive: true })
      }

      // 3. Install new version
      // Find primary file
      const file = updateData.value.files.find((f: any) => f.primary) || updateData.value.files[0]
      if (!file) throw new Error("No file found for update version")
      
      // Determine loader (fallback to fabric if not set, or keep existing)
      const loader = server.value.modpack.loader || 'fabric'
      
      const result = await installModpack(file.url, relativePath, (msg) => {
          console.log('Update progress:', msg)
      })
      
      // 4. Update metadata
      server.value.modpack.versionId = updateData.value.id
      if (updateData.value.name) server.value.modpack.versionName = updateData.value.name
      server.value.modpack.updated = new Date().toISOString()
      
       if (result && result.files) {
          server.value.files = result.files
          server.value.dependencies = result.dependencies

          // Save addons.json
          if (result.metadata && Object.keys(result.metadata).length > 0) {
             const addonsPath = await join(fullServerPath, 'addons.json')
             await writeTextFile(addonsPath, JSON.stringify(result.metadata, null, 2), { baseDir: BaseDirectory.Document })
          }
       }
      
      await saveAllSettings()
      
      showUpdateModal.value = false
      updateAvailable.value = false
      
      // Refresh UI/Data via save logic
      
   } catch (e) {
      console.error('Failed to update modpack', e)
   } finally {
      isUpdating.value = false
   }
}



// --- Crash Reports ---
interface CrashReport {
    name: string
    path: string
    created: number
    content?: string
}

const crashReports = ref<CrashReport[]>([])
const loadingReports = ref(false)
const showReportModal = ref(false)
const viewingReport = ref<CrashReport | null>(null)
const reportContent = ref('')

async function loadCrashReports() {
    loadingReports.value = true
    try {
        const folder = serverFolderName.value
        const docDir = await documentDir()
        const fullServerPath = await join(docDir, 'VoidLink', 'servers', folder)
        
        crashReports.value = await invoke('list_crash_reports_cmd', { serverPath: fullServerPath })
    } catch (e) {
        console.error('Failed to load crash reports', e)
    } finally {
        loadingReports.value = false
    }
}

async function openCrashReport(report: CrashReport) {
    viewingReport.value = report
    reportContent.value = 'Loading...'
    showReportModal.value = true
    
    try {
        reportContent.value = await invoke('read_crash_report_cmd', { path: report.path })
    } catch (e) {
        reportContent.value = `Failed to read report: ${e}`
    }
}


// --- Status Logic ---
const statusBgClass = computed(() => {
   switch (serverStatus.value) {
     case 'online': return 'bg-success-500 shadow-[0_0_10px_rgba(34,197,94,0.5)]'
     case 'starting': return 'bg-warning-500 shadow-[0_0_10px_rgba(234,179,8,0.5)]'
     case 'stopping': return 'bg-warning-500 shadow-[0_0_10px_rgba(234,179,8,0.5)]'
     default: return 'bg-neutral-500'
   }
})

function copyId() {
    if (server.value?.id) {
        navigator.clipboard.writeText(server.value.id)
            .then(() => console.log('Copied ID'))
            .catch(e => console.error('Copy failed', e))
    }
}

// --- Server Process Management Functions ---

async function startServer() {
   if (serverStatus.value !== 'offline') return
   
   // 1. Check EULA
   const folder = serverFolderName.value
   const eulaPath = `VoidLink/servers/${folder}/eula.txt`
   
   try {
      if (await exists(eulaPath, { baseDir: BaseDirectory.Document })) {
          const content = await readTextFile(eulaPath, { baseDir: BaseDirectory.Document })
          if (!content.includes('eula=true')) {
              showEulaModal.value = true
              return
          }
      } else {
          showEulaModal.value = true
          return
      }
   } catch (e) {
      console.error('EULA check failed', e)
      showEulaModal.value = true
      return
   }

   // --- Voice Chat Auto-Config ---
   try {
      if (!tunnelStore.tunnels.length) {
         await tunnelStore.fetchTunnels()
      }
      
      const voiceTunnel = tunnelStore.activeTunnels.find(t => 
         t.ports.some(p => p.protocol === 'udp' && p.label.toLowerCase().includes('voice'))
      )
      
      if (voiceTunnel) {
         const voicePort = voiceTunnel.ports.find(p => p.protocol === 'udp' && p.label.toLowerCase().includes('voice'))
         if (voicePort) {
             // Determine path based on server type
             const type = server.value?.type || 'fabric'
             const isPluginServer = ['paper', 'purpur', 'spigot', 'bukkit', 'velocity', 'folia'].includes(type)
             const configBase = isPluginServer ? 'plugins/voicechat' : 'config/voicechat'
             
             const configPath = `VoidLink/servers/${folder}/${configBase}/voicechat-server.properties`
             if (await exists(configPath, { baseDir: BaseDirectory.Document })) {
                let content = await readTextFile(configPath, { baseDir: BaseDirectory.Document })
                
                // Update port
                if (content.match(/^port=/m)) {
                   content = content.replace(/^port=.*$/m, `port=${voicePort.public_port}`)
                } else {
                   content += `\nport=${voicePort.public_port}`
                }
                
                // Update voice_host
                // voiceTunnel.full_address is usually sub.domain.com
                if (content.match(/^voice_host=/m)) {
                   content = content.replace(/^voice_host=.*$/m, `voice_host=${voiceTunnel.full_address}`)
                } else {
                   content += `\nvoice_host=${voiceTunnel.full_address}`
                }
                
                await writeTextFile(configPath, content, { baseDir: BaseDirectory.Document })
                consoleLines.value.push(`[Auto-Config] Updated Voice Chat config: Port ${voicePort.public_port}, Host ${voiceTunnel.full_address}`)
             }
         }
      }
   } catch (e) {
      console.warn('Voice Chat auto-config failed', e)
   }
   // ------------------------------

   serverStatus.value = 'starting'
   consoleLines.value.length = 0
   consoleLines.value.push('Starting server...')
   
   try {
      const folder = serverFolderName.value
      const serverPath = `VoidLink/servers/${folder}`
      const fullServerPath = await join(await documentDir(), 'VoidLink', 'servers', folder)
      
      // Detect platform
      const isWindows = navigator.userAgent.includes('Windows')
      
      // Load global settings for Java installations
      let javaInstallations: any = {}
      try {
         const settingsContent = await readTextFile('VoidLink/settings.json', { baseDir: BaseDirectory.Document })
         const globalSettings = JSON.parse(settingsContent)
         javaInstallations = globalSettings.javaInstallations || {}
      } catch (e) {
         console.log('No global settings, using default java')
      }
      
      // Auto-select Java path based on MC version
      // If path is default 'java' or empty, try auto-detection
      let javaPath = javaSettings.path
      
      if (!javaPath || javaPath === 'java') {
         await scanJava()
         
         // 1. Determine Major Version from MC version
         // < 1.17 -> 8
         // 1.17 .. 1.20.4 -> 17
         // >= 1.20.5 -> 21
         const v = server.value.version || ''
         let requiredMajor = 8
         // very naive parsing
         if (v) {
             const parts = v.split('.').map(Number)
             if (parts.length >= 2) {
                 const minor = parts[1]
                 const patch = parts[2] || 0
                 
                 if (minor >= 17) requiredMajor = 17
                 if (minor >= 20 && patch >= 5) requiredMajor = 21 // 1.20.5+
                 if (minor >= 21) requiredMajor = 21 
             }
         }
         
         const bestJava = getJavaForVersion(installations.value, requiredMajor)
         if (bestJava) {
             javaPath = bestJava.path
             consoleLines.value.push(`Auto-detected Java ${bestJava.major} (${bestJava.version}) at ${javaPath}`)
         } else {
             consoleLines.value.push(`Warning: No optimal Java found for MC ${v}. Using system default.`)
             javaPath = 'java'
         }
      } else {
         // Validating custom path
         const validation = await validateJavaPath(javaPath)
         if (!validation.is_valid) {
             consoleLines.value.push(`Warning: Configured Java path is invalid: ${validation.error || 'Unknown error'}`)
             // Fallback? Or fail? Let's just warn and try anyway or fail.
         } else {
             consoleLines.value.push(`Using Configured Java: ${validation.version} (${validation.arch})`)
         }
      }
      
      consoleLines.value.push(`Selected Java Path: ${javaPath}`)
      
      const jarPath = await join(fullServerPath, 'server.jar')
      
      // Build args array
      const javaArgs: string[] = []
      if (javaSettings.memory) {
         javaArgs.push(`-Xmx${javaSettings.memory}G`, '-Xms1G')
      }
      if (javaSettings.flags) {
         javaArgs.push(...javaSettings.flags.split(' ').filter((f: string) => f.trim()))
      }
      // Use -Duser.dir to set working directory for server files (worlds, plugins, etc)
      javaArgs.push(`-Duser.dir=${fullServerPath}`)
      javaArgs.push('-jar', 'server.jar', 'nogui')
      
      // Generate start script for manual use
      const fullJavaCmd = `"${javaPath}" ${javaArgs.join(' ')}`
      const scriptContent = isWindows 
         ? `@echo off\ncd /d "${fullServerPath}"\n${fullJavaCmd}\npause\n`
         : `#!/bin/bash\ncd "${fullServerPath}"\n${fullJavaCmd}\n`
      const scriptPath = `${serverPath}/${isWindows ? 'start.bat' : 'start.sh'}`
      await writeTextFile(scriptPath, scriptContent, { baseDir: BaseDirectory.Document })
      consoleLines.value.push(`Generated ${isWindows ? 'start.bat' : 'start.sh'}`)
      
      consoleLines.value.push(`Executing: "${javaPath}" ${javaArgs.join(' ')}`)
      
      // Use run-bat (Windows) or run-sh (Unix) to execute explicit java path
      // Because 'run-java' forces system java
      let cmd: Command
      if (isWindows) {
         // cmd /C path/to/java args...
         // Pass arguments separately. run-bat maps to `cmd` and args are passed as-is (except for /C)
         // Tauri will quote arguments that contain spaces automatically.
         cmd = Command.create('run-bat', ['/C', javaPath, ...javaArgs], {cwd: fullServerPath})
      } else {
         // sh arguments
         cmd = Command.create('run-sh', ['-c', `"${javaPath}" ${javaArgs.join(' ')}`], {cwd: fullServerPath})
      }
      
      cmd.on('close', (data) => {
         consoleLines.value.push(`Server stopped with code ${data.code}`)
         serverStatus.value = 'offline'
         serverProcess.value = null
      })
      
      cmd.on('error', (error) => {
         consoleLines.value.push(`Error: ${error}`)
         serverStatus.value = 'offline'
      })
      
      cmd.stdout.on('data', (line) => {
         // Filter RCON spam
         if (line.includes('Thread RCON Client /127.0.0.1')) return
         
         consoleLines.value.push(line)
         if (line.includes('Done') && line.includes('!')) {
            serverStatus.value = 'online'
         }
      })
      
      cmd.stderr.on('data', (line) => {
         consoleLines.value.push(line)
      })
      
      const child = await cmd.spawn()
      serverProcess.value = child
        
   } catch (e) {
      console.error('Failed to start server', e)
      consoleLines.value.push(`Failed to start: ${e}`)
      serverStatus.value = 'offline'
   }
}
async function killServer() {
   // Try using Child handle first, fallback to persisted PID
   const pid = serverStore.getPid(storeServerId.value)
   
   if (serverProcess.value) {
      await serverProcess.value.kill()
      consoleLines.value.push('Server killed by user.')
   } else if (pid) {
      // No Child handle (e.g., after F5 refresh) - use Rust kill command
      await invoke('kill_process', { pid })
      consoleLines.value.push('Server killed by user (via PID).')
   }
   
   serverStatus.value = 'offline'
}

async function acceptEula() {
   try {
      const folder = serverFolderName.value
      const eulaPath = `VoidLink/servers/${folder}/eula.txt`
      await writeTextFile(eulaPath, 'eula=true', { baseDir: BaseDirectory.Document })
      showEulaModal.value = false
      startServer()
   } catch (e) {
      console.error('Failed to accept EULA', e)
   }
}


async function stopServer() {
   if (serverStatus.value === 'offline' || !serverProcess.value) return
   
   serverStatus.value = 'stopping'
   try {
      await serverProcess.value.write('stop\n')
   } catch (e) {
      console.error('Failed to send stop command', e)
   }
}

const showDeleteModal = ref(false)
const deleteConfirmation = ref('')
const deletingServer = ref(false)

function openDeleteModal() {
   showDeleteModal.value = true
   deleteConfirmation.value = ''
}

async function confirmDeleteServer() {
   const folder = serverFolderName.value
   
   if (deleteConfirmation.value !== folder) {
      return // Button should be disabled anyway
   }

   deletingServer.value = true
   try {
      if (serverStatus.value !== 'offline' && serverProcess.value) {
         await stopServer()
         // Wait a bit for proper stop
         await new Promise(r => setTimeout(r, 2000))
      }

      const path = `VoidLink/servers/${folder}`
      await remove(path, { baseDir: BaseDirectory.Document, recursive: true })
      
      showDeleteModal.value = false
      
      // Refresh servers list in sidebar
      const serversStore = useServersStore()
      await serversStore.refreshServers()
      
      navigateTo('/')
   } catch (e) {
      console.error('Failed to delete server', e)
      // For actual errors, an alert or toast is still okay as fallback, but let's just log for now
      alert(`Failed to delete server: ${e}`) 
   } finally {
      deletingServer.value = false
   }
}

</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: #1f2937; 
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #374151; 
  border-radius: 4px;
}
</style>

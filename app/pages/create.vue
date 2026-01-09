<template>
  <div class="min-h-screen bg-gray-950 p-8 pb-32">
    <div class=" mx-auto space-y-8">
      <!-- Header with Progress -->
      <div class="relative bg-gray-900 border-b border-gray-800 pb-8 pt-6 px-8 -mx-8 -mt-8 mb-8 overflow-hidden sticky top-0 z-50">
        <!-- Background Gradients -->
        <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-neutral-900/20 via-gray-900/0 to-transparent pointer-events-none"></div>
        <div class="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-neutral-500/20 to-transparent"></div>
        <div class="absolute inset-0 backdrop-blur-xl bg-gray-900/80"></div>

        <div class="relative z-10 max-w-4xl mx-auto space-y-6">
           <div class="flex items-center gap-4">
             <UButton
               icon="i-lucide-arrow-left"
               color="neutral"
               variant="ghost"
               size="lg"
               @click="goBack"
               class="hover:bg-gray-800 text-gray-400 hover:text-white transition-colors"
             />
             <div class="flex-1">
               <h1 class="text-3xl font-bold tracking-tight text-white mb-1 flex items-center gap-3">
                 <span class="bg-gradient-to-r from-primary-400 to-primary-600 bg-clip-text text-transparent">Create Server</span>
                 <span class="text-gray-600 text-2xl font-light">/</span>
                 <span class="text-2xl font-medium text-white">{{ stepTitles[step - 1] }}</span>
               </h1>
               <p class="text-gray-400 text-sm">
                 {{ stepDescriptions[step - 1] }}
               </p>
             </div>
           </div>

           <!-- Step Progress Bar -->
           <div class="flex items-center gap-3 px-2">
             <template v-for="(title, i) in stepTitles" :key="i">
               <div 
                 class="flex items-center gap-2 cursor-pointer group"
                 @click="i < step - 1 && (step = i + 1)"
               >
                 <div 
                   class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold transition-all border-2"
                   :class="step > i + 1 
                     ? 'bg-success-500 border-success-500 text-white' 
                     : step === i + 1 
                       ? 'bg-primary-500 border-primary-500 text-white shadow-[0_0_15px_rgba(59,130,246,0.5)]' 
                       : 'bg-transparent border-gray-700 text-gray-500 group-hover:border-gray-600'"
                 >
                   <UIcon v-if="step > i + 1" name="i-lucide-check" class="w-4 h-4" />
                   <span v-else>{{ i + 1 }}</span>
                 </div>
                 <span 
                   class="text-sm font-medium transition-colors hidden sm:block"
                   :class="step === i + 1 ? 'text-white' : 'text-gray-500 group-hover:text-gray-400'"
                 >
                   {{ title }}
                 </span>
               </div>
               <div 
                 v-if="i < stepTitles.length - 1" 
                 class="flex-1 h-0.5 rounded-full transition-all duration-500"
                 :class="step > i + 1 ? 'bg-success-500/50' : 'bg-gray-800'"
               />
             </template>
           </div>
        </div>
      </div>

      <!-- Step 1: Engine Selection -->
      <div v-if="step === 1" class="max-w-5xl mx-auto animate-in fade-in duration-300">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div 
            v-for="type in serverTypes" 
            :key="type.id"
            class="relative p-6 rounded-2xl border transition-all duration-300 group cursor-pointer backdrop-blur-md"
            :class="selectedType === type.id 
              ? 'border-primary-500/50 bg-primary-900/20 ring-1 ring-primary-500/50 shadow-[0_0_30px_rgba(59,130,246,0.15)]' 
              : 'border-gray-800 bg-gray-900/40 hover:border-gray-700 hover:bg-gray-900/60'"
            @click="selectType(type.id)"
          >
            <!-- Badge -->
            <div 
              v-if="type.badge" 
              class="absolute -top-3 left-1/2 -translate-x-1/2 px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider shadow-lg border backdrop-blur-md z-10"
              :class="type.badge === 'recommended' 
                ? 'bg-blue-950/90 text-blue-400 border-blue-500/30 shadow-blue-900/20' 
                : type.badge === 'popular' ? 'bg-amber-950/90 text-amber-400 border-amber-500/30' : 'bg-green-950/90 text-green-400 border-green-500/30'"
            >
              {{ type.badge === 'recommended' ? 'Recommended' : type.badge === 'popular' ? 'Most Popular' : 'New' }}
            </div>

            <!-- Icon & Name -->
            <div class="flex flex-col items-center text-center mb-6">
              <div 
                class="w-16 h-16 rounded-2xl flex items-center justify-center mb-4 transition-transform group-hover:scale-110 duration-300 shadow-lg border border-white/5"
                :class="selectedType === type.id ? 'bg-primary-500/20' : 'bg-gray-800/50'"
              >
                <UIcon 
                  :name="type.icon" 
                  class="w-8 h-8 transition-colors" 
                  :class="selectedType === type.id ? 'text-primary-400' : 'text-gray-400 group-hover:text-gray-300'"
                />
              </div>
              <h3 class="font-bold text-xl text-white">{{ type.name }}</h3>
            </div>

            <!-- Description -->
            <p class="text-sm text-gray-400 text-center mb-6 leading-relaxed min-h-[60px] px-2">
              {{ type.description }}
            </p>

            <!-- Metadata Grid -->
            <div v-if="type.id !== 'custom'" class="space-y-4">
               <!-- Performance Bar -->
               <div class="space-y-1.5">
                  <div class="flex justify-between text-xs font-medium">
                     <span class="text-gray-500">Performance</span>
                     <span 
                        :class="{
                           'text-green-400': type.performance === 'excellent',
                           'text-blue-400': type.performance === 'good',
                           'text-amber-400': type.performance === 'moderate'
                        }"
                     >
                        {{ type.performance === 'excellent' ? 'Excellent' : type.performance === 'good' ? 'Good' : 'Moderate' }}
                     </span>
                  </div>
                  <div class="h-1.5 bg-gray-800/50 rounded-full overflow-hidden">
                     <div 
                        class="h-full rounded-full transition-all duration-500 ease-out"
                        :class="{
                           'bg-gradient-to-r from-green-500 to-emerald-400 w-full': type.performance === 'excellent',
                           'bg-gradient-to-r from-blue-500 to-indigo-400 w-3/4': type.performance === 'good',
                           'bg-gradient-to-r from-amber-500 to-orange-400 w-1/2': type.performance === 'moderate'
                        }"
                     />
                  </div>
               </div>

               <!-- Features -->
               <div class="flex items-center justify-center gap-4 py-3 border-t border-gray-800/50">
                  <UTooltip text="Plugin Support" :popper="{ arrow: true }">
                     <div class="flex items-center gap-1.5" :class="type.supportsPlugins ? 'text-gray-200' : 'text-gray-600 opacity-50'">
                        <UIcon name="i-lucide-toy-brick" class="w-4 h-4" />
                        <span class="text-xs font-medium">{{ type.supportsPlugins ? 'Plugins' : 'No Plugins' }}</span>
                     </div>
                  </UTooltip>
                  <div class="w-px h-8 bg-gray-800/50"></div>
                  <UTooltip text="Mod Support" :popper="{ arrow: true }">
                     <div class="flex items-center gap-1.5" :class="type.supportsMods ? 'text-gray-200' : 'text-gray-600 opacity-50'">
                        <UIcon name="i-lucide-puzzle" class="w-4 h-4" />
                        <span class="text-xs font-medium">{{ type.supportsMods ? 'Mods' : 'No Mods' }}</span>
                     </div>
                  </UTooltip>
               </div>
            </div>

            <!-- Select State Indicator -->
            <div 
               class="mt-2 py-2.5 rounded-xl font-medium text-sm text-center transition-all duration-300 relative overflow-hidden"
               :class="selectedType === type.id 
                  ? 'bg-primary-500 text-white shadow-lg shadow-primary-500/25' 
                  : 'bg-gray-800/50 text-gray-400 group-hover:bg-gray-800 group-hover:text-gray-200'"
            >
               <span class="relative z-10 flex items-center justify-center gap-2">
                  <UIcon v-if="selectedType === type.id" name="i-lucide-check" class="w-4 h-4" />
                  {{ selectedType === type.id ? 'Selected' : 'Select Engine' }}
               </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Step 2: Version or Modpack Selection -->
      <div v-else-if="step === 2" class="animate-in fade-in duration-300">
        
        <!-- Modpack Browser -->
        <div v-if="selectedType === 'modpack'" class="max-w-6xl mx-auto space-y-6">
          <div class="text-center space-y-2 mb-8">
             <h2 class="text-3xl font-bold text-white tracking-tight">Select Modpack</h2>
             <p class="text-gray-400 max-w-lg mx-auto">Browse thousands of community modpacks from Modrinth, or import your own .mrpack or .zip file.</p>
          </div>

          <div class="bg-gray-900/40 border border-gray-800 p-1.5 rounded-xl flex gap-1 max-w-sm mx-auto backdrop-blur-sm">
             <div 
                class="flex-1 rounded-lg py-2.5 text-sm font-medium cursor-pointer transition-all text-center"
                :class="!customModpackPath 
                  ? 'bg-primary-500 text-white shadow-lg shadow-primary-500/20' 
                  : 'text-gray-400 hover:text-white hover:bg-white/5'"
                @click="customModpackPath = null; selectedModpack = null"
             >
                Browse Modrinth
             </div>
             <div 
                class="flex-1 rounded-lg py-2.5 text-sm font-medium cursor-pointer transition-all text-center"
                :class="customModpackPath 
                  ? 'bg-primary-500 text-white shadow-lg shadow-primary-500/20' 
                  : 'text-gray-400 hover:text-white hover:bg-white/5'"
                @click="selectModpackFile"
             >
                Import File
             </div>
          </div>

          <!-- File Upload State -->
          <div v-if="customModpackPath" class="max-w-xl mx-auto mt-8">
             <div class="p-10 border-2 border-dashed border-primary-500/30 bg-primary-500/5 rounded-2xl text-center backdrop-blur-sm">
                <div class="w-20 h-20 rounded-2xl bg-primary-500/10 flex items-center justify-center mx-auto mb-6">
                   <UIcon name="i-lucide-file-archive" class="w-10 h-10 text-primary-400" />
                </div>
                <h3 class="font-bold text-xl text-white mb-2">{{ customModpackName }}</h3>
                <p class="text-sm text-gray-400 break-all mb-6">{{ customModpackPath }}</p>
                <div class="flex justify-center">
                   <UButton color="neutral" variant="outline" icon="i-lucide-refresh-cw" @click="selectModpackFile">Choose Different File</UButton>
                </div>
             </div>
          </div>

          <!-- Search & Filter -->
          <div class="space-y-4" v-if="!customModpackPath">
              <div class="flex flex-col md:flex-row gap-4 bg-gray-900/40 border border-gray-800 p-4 rounded-2xl backdrop-blur-sm">
                 <UInput 
                   v-model="modpackSearch" 
                   icon="i-lucide-search" 
                   placeholder="Search for modpacks..." 
                   class="flex-1" 
                   size="lg"
                   color="neutral"
                   variant="outline"
                 >
                   <template #trailing>
                     <UButton v-if="modpackSearch" color="neutral" variant="link" icon="i-lucide-x" :padded="false" @click="modpackSearch = ''" />
                   </template>
                 </UInput>
                 
                 <div class="flex gap-2 w-full md:w-auto overflow-x-auto pb-1 md:pb-0">
                    <div class="min-w-[140px]">
                       <USelectMenu 
                          v-model="modpackVersion" 
                          :items="availableMcVersions" 
                          placeholder="MC Version"
                          searchable
                          searchable-placeholder="Search version"
                          size="lg"
                          color="neutral"
                          variant="outline"
                       />
                    </div>

                    <div class="min-w-[120px]">
                       <USelectMenu 
                          v-model="modpackLoader" 
                          :items="availableLoaders" 
                          placeholder="Loader"
                          size="lg"
                          color="neutral"
                          variant="outline"
                       >
                          <template #label>
                             <span v-if="modpackLoader" class="capitalize">{{ modpackLoader }}</span>
                             <span v-else class="text-gray-500">Loader</span>
                          </template>
                          <template #option="{ option }">
                             <span class="capitalize">{{ option }}</span>
                          </template>
                       </USelectMenu>
                    </div>

                    <div class="min-w-[120px]">
                       <USelectMenu 
                          :items="['Relevance', 'Downloads', 'Newest']" 
                          v-model="modpackSort" 
                          size="lg"
                          color="neutral"
                          variant="outline"
                       />
                    </div>
                 </div>
              </div>

              <div class="flex justify-between items-center px-2">
                 <p class="text-sm text-gray-400">Found <span class="text-white font-medium">{{ modpackTotal.toLocaleString() }}</span> modpacks</p>
                 
                 <!-- Pagination Top (Optional / Compact) -->
              </div>
          </div>

          <!-- Loading State -->
          <div v-if="modpackLoading" class="flex flex-col items-center justify-center py-32 space-y-4">
             <UIcon name="i-lucide-loader-2" class="w-10 h-10 animate-spin text-primary-500" />
             <p class="text-gray-400 animate-pulse">Searching modpacks...</p>
          </div>

          <!-- Results Grid -->
          <div v-else-if="!customModpackPath" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-2 gap-4">
             <div 
               v-for="pack in modpacksList" 
               :key="pack.project_id"
               @click="openModpackDetails(pack)"
               class="bg-gray-900/40 hover:bg-gray-800/60 border border-gray-800 hover:border-gray-600 rounded-2xl p-5 cursor-pointer transition-all flex gap-5 group backdrop-blur-sm relative overflow-hidden"
               :class="{ 'ring-2 ring-primary-500 bg-primary-900/10': selectedModpack?.id === pack.project_id }"
             >
                <!-- Selection Overlay -->
                <div v-if="selectedModpack?.id === pack.project_id" class="absolute inset-0 bg-primary-500/5 pointer-events-none"></div>

                <div class="relative w-24 h-24 flex-shrink-0">
                   <img :src="pack.icon_url || 'https://cdn.modrinth.com/placeholder.svg'" class="w-full h-full rounded-2xl bg-gray-800 object-cover shadow-lg" loading="lazy" />
                   <div class="absolute inset-0 rounded-2xl ring-1 ring-inset ring-white/10"></div>
                </div>
                
                <div class="flex-1 min-w-0 flex flex-col justify-between py-1">
                   <div>
                      <div class="flex justify-between items-start gap-4">
                         <h3 class="font-bold text-lg text-white truncate group-hover:text-primary-400 transition-colors">{{ pack.title }}</h3>
                         <!-- Categories Pill -->
                         <div class="flex gap-1" v-if="pack.loaders && pack.loaders.length">
                            <UBadge color="neutral" variant="solid" size="xs" class="capitalize opacity-70 group-hover:opacity-100 transition-opacity">{{ pack.loaders[0] }}</UBadge>
                         </div>
                      </div>
                      <p class="text-sm text-gray-400 line-clamp-2 leading-relaxed mt-1 group-hover:text-gray-300 transition-colors pr-2">{{ pack.description }}</p>
                   </div>
                   
                   <div class="flex items-center gap-5 text-xs text-gray-500 font-medium mt-auto pt-3">
                      <span class="flex items-center gap-1.5 transition-colors group-hover:text-gray-400">
                         <UIcon name="i-lucide-download" class="w-3.5 h-3.5" />
                         {{ (pack.downloads || 0).toLocaleString() }}
                      </span>
                      <span class="flex items-center gap-1.5 transition-colors group-hover:text-gray-400">
                         <UIcon name="i-lucide-users" class="w-3.5 h-3.5" />
                         {{ (pack.followers || 0).toLocaleString() }}
                      </span>
                      <div class="flex-1"></div>
                      <div class="flex gap-1">
                         <span class="bg-gray-800/80 px-2 py-0.5 rounded text-[10px] text-gray-400 border border-gray-700/50" v-for="ver in (pack.game_versions || []).slice(0, 2)" :key="ver">{{ ver }}</span>
                         <span v-if="(pack.game_versions || []).length > 2" class="text-[10px] text-gray-500 flex items-center">+{{ pack.game_versions.length - 2 }}</span>
                      </div>
                   </div>
                </div>
             </div>
          </div>

          <!-- Pagination -->
          <div class="flex justify-center gap-2 pt-6 pb-12 items-center" v-if="totalPages > 1 && !customModpackPath">
             <UButton 
                :disabled="modpackPage === 0" 
                @click="goToPage(modpackPage)" 
                color="neutral" 
                variant="ghost" 
                icon="i-lucide-chevron-left"
                size="md"
             />
             
             <div class="flex items-center gap-1 bg-gray-900/50 rounded-lg p-1 border border-gray-800">
                <template v-for="(p, i) in visiblePages" :key="i">
                   <div v-if="p === '...'" class="w-8 h-8 flex items-center justify-center text-gray-600">...</div>
                   <UButton 
                      v-else
                      :variant="p === modpackPage + 1 ? 'solid' : 'ghost'"
                      :color="p === modpackPage + 1 ? 'primary' : 'neutral'"
                      size="sm"
                      class="min-w-[32px] justify-center text-xs font-bold"
                      @click="goToPage(p as number)"
                   >
                      {{ p }}
                   </UButton>
                </template>
             </div>

             <UButton 
                :disabled="modpackPage >= totalPages - 1" 
                @click="goToPage(modpackPage + 2)" 
                color="neutral" 
                variant="ghost" 
                icon="i-lucide-chevron-right"
                size="md"
             />
          </div>
        </div>

        <!-- Classic Version Selection -->
        <div v-else class="max-w-xl mx-auto space-y-6">
          <div class="text-center space-y-2 mb-8 animate-in slide-in-from-bottom-4 fade-in duration-500">
             <h2 class="text-3xl font-bold text-white tracking-tight">Select Version</h2>
             <p class="text-gray-400">Choose the version of Minecraft you want to install.</p>
          </div>

          <!-- Selected Engine Summary -->
          <div class="flex items-center gap-5 p-5 rounded-2xl bg-gray-900/40 border border-gray-800 backdrop-blur-sm">
            <div class="w-14 h-14 rounded-xl flex items-center justify-center bg-gray-800 shrink-0 border border-white/5 shadow-inner">
               <UIcon :name="selectedTypeData?.icon || 'i-lucide-box'" class="w-7 h-7 text-gray-300" />
            </div>
            <div>
              <p class="text-xs font-medium text-primary-400 uppercase tracking-wider mb-1">Selected Engine</p>
              <h3 class="font-bold text-xl text-white mb-0.5">{{ selectedTypeData?.name }}</h3>
              <p class="text-sm text-gray-500">{{ selectedTypeData?.description }}</p>
            </div>
            <div class="ml-auto">
               <UButton color="neutral" variant="ghost" size="sm" icon="i-lucide-arrow-left-right" @click="step = 1">Change</UButton>
            </div>
          </div>

          <!-- Custom JAR Picker -->
          <div v-if="selectedType === 'custom'" class="space-y-6">
            <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-sm">
               <label class="text-sm font-medium text-gray-200 block mb-3">Server JAR File</label>
               <div 
                 class="border-2 border-dashed rounded-xl p-8 text-center cursor-pointer transition-all duration-300 group"
                 :class="customJarPath ? 'border-success-500/50 bg-success-500/5' : 'border-gray-700 hover:border-primary-500 hover:bg-primary-500/5'"
                 @click="selectJarFile"
               >
                 <div v-if="!customJarPath" class="space-y-3 group-hover:scale-105 transition-transform duration-300">
                   <div class="w-12 h-12 rounded-full bg-gray-800 flex items-center justify-center mx-auto text-gray-400 group-hover:text-primary-400 group-hover:bg-primary-500/10 transition-colors">
                      <UIcon name="i-lucide-upload-cloud" class="w-6 h-6" />
                   </div>
                   <div>
                      <p class="font-medium text-gray-300">Click to select .jar file</p>
                      <p class="text-xs text-gray-500 mt-1">Supports Spigot, Paper, Fabric, Forge, etc.</p>
                   </div>
                 </div>
                 <div v-else class="space-y-3">
                   <div class="w-12 h-12 rounded-full bg-success-500/10 flex items-center justify-center mx-auto text-success-400">
                      <UIcon name="i-lucide-file-check" class="w-6 h-6" />
                   </div>
                   <div class="overflow-hidden">
                      <p class="font-medium text-white truncate px-4">{{ customJarName }}</p>
                      <p class="text-xs text-gray-500 truncate px-4 opacity-70">{{ customJarPath }}</p>
                   </div>
                   <UButton color="neutral" variant="subtle" size="xs" class="mt-2">Change File</UButton>
                 </div>
               </div>
            </div>

            <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-sm">
                <label class="text-sm font-medium text-gray-200 block mb-4">Capabilities</label>
                <div class="grid grid-cols-2 gap-4">
                   <div class="flex items-center justify-between bg-gray-900 border border-gray-800 p-4 rounded-xl hover:border-gray-700 transition-colors">
                      <div class="flex items-center gap-3 text-gray-300">
                         <UIcon name="i-lucide-toy-brick" class="w-5 h-5 text-gray-400" />
                         <span class="text-sm font-medium">Plugins</span>
                      </div>
                      <USwitch v-model="customSupportsPlugins" color="primary" />
                   </div>
                   <div class="flex items-center justify-between bg-gray-900 border border-gray-800 p-4 rounded-xl hover:border-gray-700 transition-colors">
                      <div class="flex items-center gap-3 text-gray-300">
                         <UIcon name="i-lucide-puzzle" class="w-5 h-5 text-gray-400" />
                         <span class="text-sm font-medium">Mods</span>
                      </div>
                      <USwitch v-model="customSupportsMods" color="primary" />
                   </div>
                </div>
            </div>
          </div>

          <!-- Version Selection -->
          <div v-else class="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-sm space-y-4">
            <div class="flex justify-between items-center">
              <label class="text-sm font-medium text-gray-200">Game Version</label>
              <div v-if="canToggleSnapshots" class="flex items-center gap-2 px-3 py-1.5 bg-gray-900 rounded-lg border border-gray-800">
                <span class="text-xs text-gray-400 font-medium">{{ toggleLabel }}</span>
                <USwitch v-model="includeSnapshots" size="sm" color="primary" />
              </div>
            </div>
            
            <USelectMenu
              v-model="selectedVersion"
              :items="availableVersions"
              :loading="isLoadingVersions"
              searchable
              searchable-placeholder="Search version..."
              placeholder="Select a version"
              size="xl"
              variant="outline"
              color="neutral"
              class="w-full"
            >
               <template #label>
                  <span v-if="selectedVersion" class="flex items-center gap-2">
                     <span class="w-2 h-2 rounded-full bg-success-500"></span>
                     <span class="font-bold text-white">{{ selectedVersion }}</span>
                  </span>
                  <span v-else class="text-gray-500">Select version...</span>
               </template>
            </USelectMenu>

            <div class="flex items-start gap-2 text-xs text-gray-500 px-1 pt-1">
               <UIcon name="i-lucide-info" class="w-4 h-4 shrink-0 text-blue-400" />
               <p>We automatically fetch the latest stable versions. Toggle snapshots if you want to test upcoming features.</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Step 3: World Settings -->
      <div v-else-if="step === 3" class="max-w-4xl mx-auto space-y-8 animate-in fade-in duration-300">
        
        <!-- World Type Cards -->
        <div class="space-y-4">
          <h3 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
             <UIcon name="i-lucide-globe" class="w-5 h-5 text-primary-400" />
             World Type
          </h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div 
              v-for="wt in worldTypes" 
              :key="wt.value"
              class="relative p-5 rounded-2xl border transition-all duration-300 cursor-pointer group bg-gray-900/40 backdrop-blur-sm"
              :class="worldType === wt.value 
                ? 'border-primary-500/50 bg-primary-900/20 ring-1 ring-primary-500/30' 
                : 'border-gray-800 hover:border-gray-600 hover:bg-gray-800/50'"
              @click="worldType = wt.value"
            >
              <!-- Selection indicator -->
              <div 
                v-if="worldType === wt.value"
                class="absolute top-3 right-3 w-5 h-5 rounded-full bg-primary-500 flex items-center justify-center shadow-lg shadow-primary-500/50"
              >
                <UIcon name="i-lucide-check" class="w-3 h-3 text-white" />
              </div>
              
              <div 
                 class="p-3 w-12 h-12 rounded-xl mb-4 flex items-center justify-center transition-colors border border-white/5"
                 :class="worldType === wt.value ? 'bg-primary-500/20 text-primary-400' : 'bg-gray-800 text-gray-400'"
              >
                <UIcon :name="wt.icon" class="w-6 h-6" />
              </div>
              <h4 class="font-bold text-base text-white mb-1.5">{{ wt.label }}</h4>
              <p class="text-xs text-gray-400 leading-relaxed">{{ wt.description }}</p>
            </div>
          </div>
        </div>

        <div class="h-px bg-gradient-to-r from-transparent via-gray-800 to-transparent w-full"></div>

        <!-- Two Column Layout -->
        <div class="grid md:grid-cols-2 gap-8">
          <!-- World Settings -->
          <div class="space-y-4">
            <h3 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
               <UIcon name="i-lucide-settings-2" class="w-5 h-5 text-primary-400" />
               Generator Settings
            </h3>
            
            <!-- World Seed -->
            <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-5 backdrop-blur-sm space-y-4">
               <div class="flex justify-between items-center">
                 <label class="text-sm font-medium text-gray-200">World Seed</label>
                 <UButton 
                   size="xs" 
                   color="primary" 
                   variant="soft"
                   icon="i-lucide-shuffle"
                   @click="worldSeed = Math.floor(Math.random() * 9999999999).toString()"
                 >
                   Randomize
                 </UButton>
               </div>
               <UInput
                 v-model="worldSeed"
                 placeholder="Leave blank for random seed"
                 size="lg"
                 class="w-full font-mono"
                 icon="i-lucide-binary"
                 color="neutral"
                 variant="outline"
               />
               <p class="text-xs text-gray-500">A seed determines how the world is generated. Same seed = same world.</p>
            </div>
          </div>

          <!-- Features -->
          <div class="space-y-4">
            <h3 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
               <UIcon name="i-lucide-sparkles" class="w-5 h-5 text-primary-400" />
               Game Rules
            </h3>
            
            <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-1 backdrop-blur-sm divide-y divide-gray-800/50">
               <!-- Generate Structures -->
               <div class="flex items-center justify-between p-4 hover:bg-gray-800/30 transition-colors rounded-t-xl">
                 <div class="flex items-start gap-3">
                   <div class="p-2 rounded-lg bg-blue-500/10 shrink-0">
                     <UIcon name="i-lucide-landmark" class="w-5 h-5 text-blue-400" />
                   </div>
                   <div>
                     <p class="font-bold text-sm text-gray-200">Generate Structures</p>
                     <p class="text-xs text-gray-500 mt-0.5">Villages, dungeons, strongholds...</p>
                   </div>
                 </div>
                 <USwitch v-model="generateStructures" color="primary" />
               </div>
   
               <!-- Hardcore Mode -->
               <div class="flex items-center justify-between p-4 hover:bg-gray-800/30 transition-colors rounded-b-xl">
                 <div class="flex items-start gap-3">
                   <div class="p-2 rounded-lg bg-red-500/10 shrink-0">
                     <UIcon name="i-lucide-skull" class="w-5 h-5 text-red-400" />
                   </div>
                   <div>
                     <p class="font-bold text-sm text-gray-200">Hardcore Mode</p>
                     <p class="text-xs text-gray-500 mt-0.5">One life only. World deleted on death.</p>
                   </div>
                 </div>
                 <USwitch v-model="hardcoreMode" color="primary" />
               </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Step 4: Server Settings -->
      <div v-else-if="step === 4" class="max-w-2xl mx-auto space-y-6 animate-in fade-in duration-300">
        <!-- Java Version Info -->
        <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-sm relative overflow-hidden">
           <div class="absolute top-0 right-0 w-32 h-32 bg-amber-500/5 rounded-full blur-3xl pointer-events-none"></div>
           
           <div class="flex items-center justify-between mb-6">
              <div class="flex items-center gap-3">
                 <div class="p-2.5 bg-amber-500/10 rounded-xl">
                    <UIcon name="i-lucide-coffee" class="w-5 h-5 text-amber-500" />
                 </div>
                 <h3 class="font-bold text-lg text-white">Java Runtime</h3>
              </div>
              <UBadge color="neutral" variant="soft" size="md" class="font-mono">
                 <UIcon name="i-lucide-terminal" class="w-3 h-3 mr-1" />
                 {{ detectedJavaVersion || 'Checking...' }}
              </UBadge>
           </div>

           <div class="p-4 rounded-xl border transition-all" :class="javaCompatibility.class">
             <div class="flex items-start gap-3">
               <UIcon :name="javaCompatibility.icon" class="w-5 h-5 shrink-0 mt-0.5" />
               <div class="flex-1">
                 <p class="font-bold text-sm mb-1">{{ javaCompatibility.title }}</p>
                 <p class="text-xs opacity-90 leading-relaxed">{{ javaCompatibility.message }}</p>
               </div>
               <UButton 
                  v-if="!detectedJavaVersion"
                  icon="i-lucide-refresh-cw" 
                  size="xs" 
                  color="neutral" 
                  variant="ghost"
                  :loading="checkingJava"
                  @click="detectJavaVersion"
               />
             </div>
           </div>
        </div>

        <!-- Server Name & RAM -->
        <div class="bg-gray-900/40 border border-gray-800 rounded-2xl p-8 backdrop-blur-sm space-y-8 relative overflow-hidden">
           <div class="absolute -top-10 -left-10 w-40 h-40 bg-primary-500/5 rounded-full blur-3xl pointer-events-none"></div>

           <h3 class="font-bold text-xl text-white flex items-center gap-2">
              <UIcon name="i-lucide-slider" class="w-5 h-5 text-primary-400" />
              Configuration
           </h3>
           
           <!-- Server Name -->
           <div class="space-y-3 flex flex-col">
             <label class="text-sm font-medium text-gray-200">Server Name</label>
             <UInput
               v-model="serverName"
               placeholder="My Awesome Server"
               size="xl"
               icon="i-lucide-type"
               color="neutral"
               variant="outline"
             />
             <p class="text-xs text-gray-500">The folder name will be generated automatically based on this name.</p>
           </div>

           <!-- RAM Limit -->
           <div class="space-y-6 pt-4 border-t border-gray-800/50">
             <div class="flex justify-between items-end">
               <div>
                  <label class="text-sm font-medium text-gray-200 flex items-center gap-2">
                     <UIcon name="i-lucide-memory-stick" class="w-4 h-4 text-gray-400" />
                     RAM Allocation
                  </label>
                  <p class="text-xs text-gray-500 mt-1">Memory dedicated to the server process.</p>
               </div>
               <div class="text-right">
                  <span class="text-3xl font-bold bg-gradient-to-r from-primary-400 to-primary-600 bg-clip-text text-transparent">{{ ramLimit }}</span>
                  <span class="text-sm text-gray-500 font-medium ml-1">GB</span>
               </div>
             </div>
             
             <div class="relative px-2">
               <input 
                 type="range" 
                 v-model.number="ramLimit" 
                 min="1" 
                 :max="systemRamGB" 
                 step="1"
                 class="w-full h-3 bg-gray-800 rounded-full appearance-none cursor-pointer accent-primary-500 hover:accent-primary-400 focus:outline-none focus:ring-2 focus:ring-primary-500/50 transition-all"
               />
               <div class="flex justify-between text-xs font-mono text-gray-500 mt-3 px-1">
                 <span>1 GB</span>
                 <span class="text-primary-500 font-bold">Recommended: 6-8 GB</span>
                 <span>{{ systemRamGB }} GB</span>
               </div>
             </div>
           </div>

           <!-- Path Preview -->
           <div class="space-y-3 pt-4 border-t border-gray-800/50 opacity-70 hover:opacity-100 transition-opacity">
             <label class="text-xs font-medium text-gray-500 uppercase tracking-wider">Installation Path Preview</label>
             <div class="relative">
                <UInput
                  :model-value="serverPath"
                  readonly
                  icon="i-lucide-folder-open"
                  color="neutral"
                  variant="subtle"
                  size="sm"
                  class="font-mono text-xs cursor-default w-full"
                />
             </div>
           </div>
        </div>
      </div>

      <!-- Step 5: Summary -->
      <div v-else-if="step === 5" class="max-w-xl mx-auto space-y-6 animate-in fade-in duration-300">
        <div class="bg-gray-900/40 border border-gray-800 rounded-2xl overflow-hidden backdrop-blur-sm">
           <div class="bg-gray-900/50 border-b border-gray-800/80 p-6 flex flex-col items-center text-center">
              <div class="w-16 h-16 rounded-2xl bg-success-500/10 flex items-center justify-center mb-4 ring-1 ring-success-500/20 shadow-[0_0_30px_rgba(34,197,94,0.1)]">
                 <UIcon name="i-lucide-rocket" class="w-8 h-8 text-success-500" />
              </div>
              <h2 class="text-2xl font-bold text-white mb-1">Ready to Launch!</h2>
              <p class="text-gray-400 text-sm">Review your configuration before creating the server.</p>
           </div>
           
           <div class="divide-y divide-gray-800/50">
             <div v-if="selectedType === 'modpack'" class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Modpack</span>
               <span class="font-bold text-primary-400 text-right">{{ selectedModpack?.title }}</span>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Engine</span>
               <div class="flex items-center gap-2">
                  <UIcon :name="selectedTypeData?.icon" class="w-4 h-4 text-gray-400" />
                  <span class="font-medium text-white">{{ selectedTypeData?.name }}</span>
               </div>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Version</span>
               <UBadge color="neutral" variant="soft" class="font-bold">{{ selectedType === 'custom' ? customJarName : selectedVersion }}</UBadge>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">World Type</span>
               <span class="font-medium text-white">{{ worldTypes.find(t => t.value === worldType)?.label }}</span>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Seed</span>
               <span class="font-mono text-xs text-gray-300 bg-gray-950 px-2 py-1 rounded">{{ worldSeed || 'Random' }}</span>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Server Name</span>
               <span class="font-medium text-white">{{ serverName }}</span>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">RAM Allocation</span>
               <span class="font-bold text-white">{{ ramLimit }} GB</span>
             </div>
             
             <div class="p-4 flex justify-between items-center hover:bg-white/5 transition-colors">
               <span class="text-gray-500 text-sm font-medium">Java Version</span>
               <span class="font-mono text-xs text-gray-400">{{ detectedJavaVersion || 'System default' }}</span>
             </div>
           </div>
        </div>

        <!-- Status Message -->
        <div v-if="statusMessage" class="text-center animate-in fade-in slide-in-from-bottom-2">
          <span class="inline-flex items-center gap-3 px-5 py-2.5 rounded-full bg-gray-900 border border-gray-700 text-white text-sm font-medium shadow-lg">
            <UIcon name="i-lucide-loader-2" class="w-4 h-4 animate-spin text-primary-500" />
            {{ statusMessage }}
          </span>
        </div>
      </div>

      <!-- Footer Actions -->
      <div class="fixed bottom-0 left-0 right-0 p-6 bg-gray-900/80 backdrop-blur-xl border-t border-gray-800 z-50">
        <div class="max-w-4xl mx-auto flex justify-between items-center">
          <UButton
            v-if="step > 1"
            color="neutral"
            variant="ghost"
            icon="i-lucide-arrow-left"
            size="lg"
            @click="step--"
          >
            Back
          </UButton>
          <div v-else />

          <UButton
            v-if="step < 5"
            color="primary"
            trailing-icon="i-lucide-arrow-right"
            size="lg"
            :disabled="!canContinue"
            @click="step++"
          >
            Continue
          </UButton>

          <UButton
            v-else
            color="primary"
            trailing-icon="i-lucide-rocket"
            size="lg"
            :loading="isCreating"
            :disabled="!isValid"
            @click="createServer"
            class="font-bold px-8 shadow-lg shadow-primary-500/25"
          >
            {{ isCreating ? 'Creating...' : 'Launch Server' }}
          </UButton>
        </div>
      </div>
    </div>
    <!-- Creation Overlay -->
    <div v-if="isCreating && selectedType === 'modpack'" class="fixed inset-0 bg-black/80 backdrop-blur-sm z-[9999] flex flex-col items-center justify-center p-8">
       <UIcon name="i-lucide-package" class="w-16 h-16 text-primary-500 mb-4 animate-bounce" />
       <h2 class="text-2xl font-bold bg-gradient-to-r from-primary-400 to-secondary-400 bg-clip-text text-transparent mb-2">Installing Modpack...</h2>
       <p class="text-gray-400 mb-8 max-w-md text-center">{{ statusMessage }}</p>
       
       <div class="w-full max-w-sm">
          <UProgress :value="creationProgress" color="primary" indicator class="mb-2" />
          <div class="flex justify-between text-xs text-gray-500">
             <span>{{ Math.round(creationProgress) }}%</span>
             <span>Minetools Installer</span>
          </div>
       </div>
    </div>
      <!-- Modpack Detail Modal -->
      <UModal v-model:open="showModpackModal" fullscreen>

        <template #header>
          <div class="w-full flex items-start gap-5 p-6">
            <div class="w-20 h-20 rounded-xl bg-gray-800 flex-shrink-0 overflow-hidden shadow-lg border border-gray-700">
                <img v-if="detailedModpack?.icon_url" :src="detailedModpack.icon_url" class="w-full h-full object-cover" />
                <div v-else class="w-full h-full flex items-center justify-center">
                  <UIcon name="i-lucide-package" class="w-10 h-10 text-gray-500" />
                </div>
            </div>
            
            <div class="flex-1">
                <h2 class="text-2xl font-bold flex items-center gap-3">
                  {{ detailedModpack?.title }}
                  <UBadge color="neutral" variant="soft" class="capitalize" :ui="{ base: 'rounded-full' }">{{ detailedModpack?.project_type }}</UBadge>
                </h2>
                <p class="text-gray-400 mt-1">{{ detailedModpack?.description }}</p>
                
                <div class="flex items-center gap-6 mt-4 text-sm text-gray-400">
                  <div class="flex items-center gap-1.5" title="Downloads">
                      <UIcon name="i-lucide-download" class="w-4 h-4" />
                      <span>{{ detailedModpack?.downloads?.toLocaleString() }}</span>
                  </div>
                  <div class="flex items-center gap-1.5" title="Followers">
                      <UIcon name="i-lucide-heart" class="w-4 h-4" />
                      <span>{{ detailedModpack?.followers?.toLocaleString() }}</span>
                  </div>
                  <div class="flex items-center gap-1.5" title="Updated">
                      <UIcon name="i-lucide-calendar" class="w-4 h-4" />
                      <span>{{ new Date(detailedModpack?.updated).toLocaleDateString() }}</span>
                  </div>
                </div>
            </div>

            <UButton 
              icon="i-lucide-x" 
              color="neutral" 
              variant="ghost" 
              @click="showModpackModal = false"
            />
          </div>
        </template>

        <template #body>
          <div class="flex flex-col h-full bg-gray-950 text-white">
              <!-- Modal Content (Scrollable) -->
              <div class="flex-1 overflow-y-auto p-6 md:p-8 custom-scrollbar">
                <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-4 gap-8">
                    
                    <!-- Left: Description -->
                    <div class="lg:col-span-3">
                      <div v-if="loadingDetails" class="flex justify-center py-20">
                          <UIcon name="i-lucide-loader-2" class="w-10 h-10 animate-spin text-primary-500" />
                      </div>
                      <div v-else>
                          <h3 class="text-xl font-bold mb-4 flex items-center gap-2">
                              <UIcon name="i-lucide-file-text" class="w-5 h-5 text-gray-500" />
                              Description
                          </h3>
                          <div 
                            class="prose prose-invert prose-indigo max-w-none bg-gray-900/30 p-6 rounded-xl border border-gray-800"
                            v-html="modpackDescriptionHtml || 'No description provided.'"
                          ></div>
                      </div>
                    </div>

                    <!-- Right: Sidebar -->
                    <div class="lg:col-span-1 space-y-6">
                      <!-- Version Selector -->
                      <div class="bg-gray-900 border border-gray-800 rounded-xl p-4">
                          <h3 class="font-bold mb-3 flex items-center gap-2 text-primary-400">
                            <UIcon name="i-lucide-history" class="w-4 h-4" />
                            Select Version
                          </h3>
                          
                          <div class="space-y-4" v-if="modpackVersions.length">
                            <USelectMenu
                                v-model="selectedModpackVersion"
                                :items="modpackVersions"
                                option-attribute="name"
                                searchable
                                searchable-placeholder="Search version..."
                                placeholder="Select a version"
                                class="w-full"
                            >
                                <template #item="{ item }">
                                  <div class="flex flex-col gap-0.5 truncate py-1">
                                      <span class="truncate font-medium">{{ item.name }}</span>
                                      <span class="text-xs text-gray-500 flex items-center gap-2">
                                        <span>{{ item.version_number }}</span>
                                        <span>•</span>
                                        <span>{{ new Date(item.date_published).toLocaleDateString() }}</span>
                                      </span>
                                  </div>
                                </template>
                            </USelectMenu>
                            
                            <div v-if="selectedModpackVersion" class="space-y-2 text-xs text-gray-400 bg-gray-950/50 p-3 rounded border border-gray-800">
                                <div class="flex justify-between">
                                  <span>Minecraft</span>
                                  <span class="text-white">{{ selectedModpackVersion.game_versions.join(', ') }}</span>
                                </div>
                                <div class="flex justify-between">
                                  <span>Loader</span>
                                  <span class="text-white capitalize">{{ selectedModpackVersion.loaders.join(', ') }}</span>
                                </div>
                                <div class="flex justify-between">
                                  <span>Type</span>
                                  <span :class="{
                                      'text-green-400': selectedModpackVersion.version_type === 'release',
                                      'text-yellow-400': selectedModpackVersion.version_type === 'beta',
                                      'text-red-400': selectedModpackVersion.version_type === 'alpha'
                                  }" class="capitalize">{{ selectedModpackVersion.version_type }}</span>
                                </div>
                                <div class="flex justify-between">
                                  <span>Files</span>
                                  <span class="text-white">{{ selectedModpackVersion.files?.length || 0 }}</span>
                                </div>
                            </div>
                          </div>
                          <div v-else class="text-gray-500 text-sm">No versions found</div>
                      </div>
                      
                      <!-- Categories -->
                      <div v-if="detailedModpack?.categories?.length" class="bg-gray-900 border border-gray-800 rounded-xl p-4">
                          <h3 class="font-bold mb-3 flex items-center gap-2 text-primary-400">
                            <UIcon name="i-lucide-tag" class="w-4 h-4" />
                            Categories
                          </h3>
                          <div class="flex flex-wrap gap-2">
                            <UBadge v-for="cat in detailedModpack.categories" :key="cat" color="neutral" variant="soft" class="capitalize">
                                {{ cat }}
                            </UBadge>
                          </div>
                      </div>
                      
                      <!-- License -->
                      <div v-if="detailedModpack?.license" class="bg-gray-900 border border-gray-800 rounded-xl p-4">
                          <h3 class="font-bold mb-3 flex items-center gap-2 text-primary-400">
                            <UIcon name="i-lucide-scale" class="w-4 h-4" />
                            License
                          </h3>
                          <p class="text-sm text-gray-300">{{ detailedModpack.license.name || detailedModpack.license.id }}</p>
                      </div>

                    </div>
                </div>
              </div>

              <!-- Footer Actions -->
              <div class="p-4 border-t border-gray-800 bg-gray-900 flex justify-end gap-3 rounded-b-lg">
                <UButton color="neutral" variant="ghost" size="lg" @click="showModpackModal = false">
                    Cancel
                </UButton>
                <UButton 
                    color="primary" 
                    size="lg" 
                    icon="i-lucide-check" 
                    @click="confirmModpackSelection"
                    :disabled="!selectedModpackVersion"
                >
                    Confirm Selection
                </UButton>
              </div>
          </div>
        </template>
      </UModal>

  </div>
</template>

<script setup lang="ts">
import { v4 as uuidv4 } from 'uuid' 
import { fetch } from '@tauri-apps/plugin-http'
import { mkdir, writeFile, writeTextFile, readTextFile, copyFile, BaseDirectory } from '@tauri-apps/plugin-fs'
import { join, documentDir } from '@tauri-apps/api/path'
import { open } from '@tauri-apps/plugin-dialog'
import { Command } from '@tauri-apps/plugin-shell'
import MarkdownIt from 'markdown-it'

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true
})

// Configure links to open in new tab
const defaultRender = md.renderer.rules.link_open || function(tokens, idx, options, env, self) {
  return self.renderToken(tokens, idx, options)
}

md.renderer.rules.link_open = function (tokens, idx, options, env, self) {
  const aIndex = tokens[idx].attrIndex('target')

  if (aIndex < 0) {
    tokens[idx].attrPush(['target', '_blank'])
  } else {
    tokens[idx].attrs![aIndex][1] = '_blank'
  }
  
  // Add rel="noopener noreferrer" for security
  const relIndex = tokens[idx].attrIndex('rel')
  if (relIndex < 0) {
    tokens[idx].attrPush(['rel', 'noopener noreferrer']) 
  } else {
    tokens[idx].attrs![relIndex][1] = 'noopener noreferrer'
  }

  return defaultRender(tokens, idx, options, env, self)
}

// Step state
const step = ref(1)

const stepTitles = ['Engine', 'Version', 'World', 'Settings', 'Launch']
const stepDescriptions = [
  'Choose the server software that powers your world',
  'Select the game version to run',
  'Configure your world generation settings',
  'Set up server name, memory, and Java',
  'Review and launch your new server'
]

// Engine state
const selectedType = ref<string | null>(null)
const customJarPath = ref<string | null>(null)
const customJarName = ref('')
const customSupportsPlugins = ref(false)
const customSupportsMods = ref(false)

// Version state
const selectedVersion = ref<string | undefined>(undefined)
const isLoadingVersions = ref(false)
const availableVersions = ref<string[]>([])
const includeSnapshots = ref(false)

// World state
const worldSeed = ref('')
const worldType = ref('default')
const levelName = ref('world')
const generateStructures = ref(true)
const hardcoreMode = ref(false)

const worldTypes = [
  { label: 'Default', value: 'default', icon: 'i-lucide-mountain', description: 'Standard Minecraft terrain generation suitable for most gameplay.' },
  { label: 'Superflat', value: 'flat', icon: 'i-lucide-minus', description: 'Completely flat terrain, ideal for creative building projects.' },
  { label: 'Large Biomes', value: 'largeBiomes', icon: 'i-lucide-globe', description: 'Standard terrain but with biomes expanded to 16x larger size.' },
  { label: 'Amplified', value: 'amplified', icon: 'i-lucide-trending-up', description: 'Exaggerated terrain height and cliffs (requires strong CPU).' }
]

// Server settings state
const serverName = ref('')
const ramLimit = ref(6)

// Java detection
const detectedJavaVersion = ref<string | null>(null)
const checkingJava = ref(false)
import { useJava } from '~/composables/useJava'
const { installations, scanJava, getJavaForVersion } = useJava()

import { installModpack, installMrpack, installZip, readMrpackMetadata, readMmcMetadata } from '~/utils/modpack'

// ... existing imports ...

// Creation state
const isCreating = ref(false)
const statusMessage = ref('')
const creationProgress = ref(0) // New progress state

// Modpack state
const modpackSearch = ref('')
const modpacksList = ref<any[]>([])
const modpackLoading = ref(false)
const selectedModpack = ref<any>(null)
const customModpackPath = ref<string | null>(null)
const customModpackName = ref<string>('')
const customModpackMetadata = ref<any>(null) // New metadata state
const selectedModpackVersionData = ref<any>(null) // New version data state
const modpackPage = ref(0)
const modpackTotal = ref(0)
// Filters
const modpackSort = ref('Relevance')
const modpackVersion = ref<string | undefined>(undefined)
const modpackLoader = ref<string | undefined>(undefined)

// Modpack Details Modal State
const showModpackModal = ref(false)
const detailedModpack = ref<any>(null)
const modpackVersions = ref<any[]>([])
const selectedModpackVersion = ref<any>(null)
const modpackDescriptionHtml = ref('')
const loadingDetails = ref(false)

const availableMcVersions = ref<string[]>([])
const availableLoaders = ['fabric', 'forge', 'neoforge', 'quilt']
const systemRamGB = ref(16) // Default fallback

// Fetch system RAM on init
;(async () => {
   try {
      const { invoke } = await import('@tauri-apps/api/core')
      const sysInfo = await invoke<{ total_memory_bytes: number }>('get_system_info')
      systemRamGB.value = Math.floor(sysInfo.total_memory_bytes / (1024 * 1024 * 1024))
   } catch (e) {
      console.log('Failed to get system info, using 16GB default')
   }
})()

async function fetchModrinthVersions() {
  try {
    const res = await fetch('https://api.modrinth.com/v2/tag/game_version')
    const data: Array<{ version: string, version_type: string, date: string }> = await res.json()
    
    availableMcVersions.value = data
      .filter(v => v.version_type === 'release')
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
      .map(v => v.version)
  } catch (e) {
    console.error('Failed to resolve modrinth versions', e)
    availableMcVersions.value = ['1.21.1', '1.21', '1.20.6', '1.20.4', '1.20.1', '1.19.2', '1.18.2', '1.16.5', '1.12.2']
  }
}

fetchModrinthVersions()

interface ServerType {
  id: string
  name: string
  icon: string
  description: string
  performance: 'excellent' | 'good' | 'moderate'
  supportsPlugins: boolean
  supportsMods: boolean
  badge?: 'recommended' | 'popular' | 'new'
  api: {
    url: string
    type: 'mcjarfiles' | 'papermc'
    snapshotUrl?: string
    previewUrl?: string
  }
}

const serverTypes: ServerType[] = [
  { 
    id: 'vanilla', 
    name: 'Vanilla', 
    icon: 'i-lucide-box', 
    description: 'The unmodified, official Minecraft experience as provided by Mojang.',
    performance: 'good',
    supportsPlugins: false,
    supportsMods: false,
    badge: 'recommended',
    api: { 
      url: 'https://mcjarfiles.com/api/get-versions/vanilla/release',
      snapshotUrl: 'https://mcjarfiles.com/api/get-versions/vanilla/snapshot',
      type: 'mcjarfiles'
    }
  },
  { 
    id: 'paper', 
    name: 'Paper', 
    icon: 'i-lucide-scroll', 
    description: 'An optimized fork of Spigot, offering better performance and configuration. Best for plugin servers.',
    performance: 'excellent',
    supportsPlugins: true,
    supportsMods: false,
    badge: 'popular',
    api: { url: 'https://mcjarfiles.com/api/get-versions/servers/paper', type: 'mcjarfiles' }
  },
  { 
    id: 'modpack', 
    name: 'Modpack', 
    icon: 'i-lucide-package', 
    description: 'A pre-configured server with a collection of mods for a specific game experience. Browse modrinth or import your own.',
    performance: 'good',
    supportsPlugins: false,
    supportsMods: true,
    badge: 'new',
    api: { url: '', type: 'custom' as any }
  },
  { 
    id: 'purpur', 
    name: 'Purpur', 
    icon: 'i-lucide-cat', 
    description: 'Drop-in replacement for Paper with many additional gameplay features and tweaks.',
    performance: 'excellent',
    supportsPlugins: true,
    supportsMods: false,
    api: { url: 'https://mcjarfiles.com/api/get-versions/servers/purpur', type: 'mcjarfiles' }
  },
  { 
    id: 'fabric', 
    name: 'Fabric', 
    icon: 'i-lucide-layers', 
    description: 'Lightweight, modern modding toolchain. Fast updates and performance-focused mods.',
    performance: 'good',
    supportsPlugins: false,
    supportsMods: true,
    api: { url: 'https://mcjarfiles.com/api/get-versions/modded/fabric', type: 'mcjarfiles' }
  },
  { 
    id: 'forge', 
    name: 'Forge', 
    icon: 'i-lucide-anvil', 
    description: 'The original and most popular mod loader. Largest mod library available.',
    performance: 'moderate',
    supportsPlugins: false,
    supportsMods: true,
    api: { url: 'https://mcjarfiles.com/api/get-versions/modded/forge', type: 'mcjarfiles' }
  },
  { 
    id: 'neoforge', 
    name: 'NeoForge', 
    icon: 'i-lucide-zap', 
    description: 'Community-driven fork of Forge with improved performance and modern features.',
    performance: 'good',
    supportsPlugins: false,
    supportsMods: true,
    api: { url: 'https://mcjarfiles.com/api/get-versions/modded/neoforge', type: 'mcjarfiles' }
  },
  { 
    id: 'velocity', 
    name: 'Velocity', 
    icon: 'i-lucide-network', 
    description: 'A modern, high-performance proxy for connecting multiple servers together.',
    performance: 'excellent',
    supportsPlugins: true,
    supportsMods: false,
    api: { url: 'https://mcjarfiles.com/api/get-versions/proxies/velocity', type: 'mcjarfiles' }
  },
  { 
    id: 'custom', 
    name: 'Custom JAR', 
    icon: 'i-lucide-upload', 
    description: 'Use your own server JAR file from local disk.',
    performance: 'good',
    supportsPlugins: customSupportsPlugins.value,
    supportsMods: customSupportsMods.value,
    api: { url: '', type: 'custom' as any }
  }
]

const selectedTypeData = computed(() => serverTypes.find(t => t.id === selectedType.value))

const canToggleSnapshots = computed(() => {
  return selectedTypeData.value?.id === 'vanilla'
})

const toggleLabel = computed(() => 'Show Snapshots')

// Java compatibility check
const javaCompatibility = computed(() => {
  if (!detectedJavaVersion.value) {
    return {
      class: 'bg-gray-800 text-gray-400',
      icon: 'i-lucide-info',
      title: 'Detecting Java...',
      message: 'Checking your system for Java installation'
    }
  }

  const javaMatch = String(detectedJavaVersion.value || '').match(/(\d+)/)
  const javaVer = javaMatch ? parseInt(javaMatch[1]) : 0
  const mcVersion = selectedVersion.value || ''
  
  // Determine required Java version
  let requiredJava = 17
  if (mcVersion >= '1.20.5') requiredJava = 21
  else if (mcVersion >= '1.17') requiredJava = 17
  else requiredJava = 8

  if (javaVer >= requiredJava) {
    return {
      class: 'bg-success-900/30 text-success-400',
      icon: 'i-lucide-check-circle',
      title: 'Java Compatible',
      message: `Java ${javaVer} meets the requirement of Java ${requiredJava}+ for Minecraft ${mcVersion || 'this version'}`
    }
  } else {
    return {
      class: 'bg-warning-900/30 text-warning-400',
      icon: 'i-lucide-alert-triangle',
      title: 'Java Update Recommended',
      message: `Minecraft ${mcVersion} requires Java ${requiredJava}+. You have Java ${javaVer}.`
    }
  }
})

const serverId = computed(() => uuidv4().slice(0, 8))

const serverFolderName = computed(() => {
  const safeName = (serverName.value || 'untitled')
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/(^-|-$)/g, '')
  return `${safeName}-${serverId.value}`
})

const serverPath = computed(() => {
  return `~/Documents/VoidLink/servers/${serverFolderName.value}`
})

async function searchModpacks(query = '') {
  modpackLoading.value = true
  try {
    const facets = [['project_type:modpack']]
    
    if (modpackVersion.value) {
      facets.push([`versions:${modpackVersion.value}`])
    }
    
    if (modpackLoader.value) {
      facets.push([`categories:${modpackLoader.value}`])
    }

    let index = 'relevance'
    if (modpackSort.value === 'Downloads') index = 'downloads'
    if (modpackSort.value === 'Newest') index = 'newest'

    const params = new URLSearchParams({
      query,
      facets: JSON.stringify(facets),
      index,
      limit: '20',
      offset: (modpackPage.value * 20).toString()
    })
    
    // Using Modrinth API directly
    const res = await fetch(`https://api.modrinth.com/v2/search?${params}`)
    const data = await res.json()
    modpacksList.value = data.hits
    modpackTotal.value = data.total_hits
  } catch (e) {
    console.error('Failed to search modpacks', e)
  } finally {
    modpackLoading.value = false
  }
}

// Debounce search input
let searchTimeout: any = null
watch(modpackSearch, () => {
  modpackPage.value = 0
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    searchModpacks(modpackSearch.value)
  }, 500)
})

// Immediate search for filters
watch([modpackSort, modpackVersion, modpackLoader], () => {
  modpackPage.value = 0
  searchModpacks(modpackSearch.value)
})

watch(() => step.value, (newStep) => {
  if (newStep === 2 && selectedType.value === 'modpack' && modpacksList.value.length === 0) {
    searchModpacks()
  }
})

// Replaced by openModpackDetails
// async function selectModpack(pack: any) { ... }

async function openModpackDetails(pack: any) {
  detailedModpack.value = pack
  showModpackModal.value = true
  loadingDetails.value = true
  modpackVersions.value = []
  selectedModpackVersion.value = null
  modpackDescriptionHtml.value = ''

  try {
     // Fetch full project details to get body
     const projectRes = await fetch(`https://api.modrinth.com/v2/project/${pack.project_id || pack.id || pack.slug}`)
     const project = await projectRes.json()
     detailedModpack.value = { ...pack, ...project } // Merge details
     
     // Render description
     if (project.body) {
        modpackDescriptionHtml.value = md.render(project.body)
     }

     // Fetch versions
     const versionsRes = await fetch(`https://api.modrinth.com/v2/project/${pack.project_id || pack.id || pack.slug}/version`)
     const versions = await versionsRes.json()
     modpackVersions.value = versions

     // Pre-select latest release
     if (versions.length > 0) {
        const latest = versions.find((v: any) => v.version_type === 'release') || versions[0]
        selectedModpackVersion.value = latest
     }

  } catch (e) {
     console.error('Failed to load modpack details', e)
  } finally {
     loadingDetails.value = false
  }
}

function confirmModpackSelection() {
   if (!selectedModpackVersion.value) return
   
   // Apply selection
   selectedModpack.value = detailedModpack.value
   selectedModpackVersionData.value = selectedModpackVersion.value
   
   // Set game version from selected modpack version
   if (selectedModpackVersion.value.game_versions && selectedModpackVersion.value.game_versions.length > 0) {
      selectedVersion.value = selectedModpackVersion.value.game_versions[0]
   }
   
   // Set server name if empty
   if (!serverName.value && detailedModpack.value.title) {
      serverName.value = detailedModpack.value.title
   }

   showModpackModal.value = false
   step.value++
}

// Keeping original selectModpack for basic selection if needed, or deprecating it.
// We can just leave it as is, but UI now calls openModpackDetails.
async function selectModpack(pack: any) {
  // Redirect to modal flow
  openModpackDetails(pack)
}

// Pagination Logic
const totalPages = computed(() => Math.ceil(modpackTotal.value / 20))

const visiblePages = computed(() => {
  const current = modpackPage.value + 1
  const total = totalPages.value
  if (total <= 1) return [1]
  
  const delta = 2
  const range: (number | string)[] = []
  
  for (let i = Math.max(2, current - delta); i <= Math.min(total - 1, current + delta); i++) {
    range.push(i)
  }
  
  if (current - delta > 2) {
    range.unshift('...')
  }
  
  if (current + delta < total - 1) {
    range.push('...')
  }
  
  range.unshift(1)
  if (total > 1) {
    range.push(total)
  }
  
  return range
})

function goToPage(p: number | string) {
   if (typeof p !== 'number') return
   modpackPage.value = p - 1
   searchModpacks(modpackSearch.value)
}

const canContinue = computed(() => {
  switch (step.value) {
    case 1: return !!selectedType.value
    case 2: 
      if (selectedType.value === 'modpack') return !!selectedModpack.value
      if (selectedType.value === 'custom') return !!customJarPath.value
      return !!selectedVersion.value
    case 3: return true // World settings are optional
    case 4: return serverName.value.length > 0
    default: return true
  }
})

const isValid = computed(() => {
  if (selectedType.value === 'custom') {
    return customJarPath.value && serverName.value.length > 0
  }
  return selectedVersion.value && serverName.value.length > 0
})

function selectType(id: string) {
  selectedType.value = id
}

async function selectJarFile() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'JAR Files', extensions: ['jar'] }]
  })
  
  if (selected && typeof selected === 'string') {
    customJarPath.value = selected
    const parts = selected.replace(/\\/g, '/').split('/')
    customJarName.value = parts[parts.length - 1] || 'server.jar'
  }
}

async function selectModpackFile() {
  const selected = await open({
    multiple: false,
    filters: [
      { name: 'Modpack Files', extensions: ['mrpack', 'zip'] }
    ]
  })
  
  if (selected && typeof selected === 'string') {
    customModpackPath.value = selected
    const parts = selected.replace(/\\/g, '/').split('/')
    customModpackName.value = parts[parts.length - 1] || 'modpack.mrpack'
    
    // Auto-detect metadata for .mrpack
    if (customModpackPath.value.endsWith('.mrpack')) {
       try {
          const { readFile } = await import('@tauri-apps/plugin-fs')
          const fileData = await readFile(customModpackPath.value)
          const meta = await readMrpackMetadata(fileData)
          
          customModpackMetadata.value = meta
          if (meta.name) customModpackName.value = meta.name
          if (meta.dependencies.minecraft) {
             selectedVersion.value = meta.dependencies.minecraft
          }
          
          // Priority: fabric, quilt, neoforge, forge
          if (meta.dependencies["fabric-loader"]) modpackLoader.value = 'fabric'
          else if (meta.dependencies["quilt-loader"]) modpackLoader.value = 'quilt'
          else if (meta.dependencies["neoforge"]) modpackLoader.value = 'neoforge'
          else if (meta.dependencies["forge"]) modpackLoader.value = 'forge'
          
       } catch (e) {
          console.error('Failed to parse mrpack metadata', e)
       }
    } else if (customModpackPath.value.endsWith('.zip')) {
        // Auto-detect metadata for MultiMC/Prism Export (.zip)
        try {
            const { readFile } = await import('@tauri-apps/plugin-fs')
            const fileData = await readFile(customModpackPath.value)
            const mmcMeta = await readMmcMetadata(fileData)
            
            customModpackMetadata.value = mmcMeta
            
            // Extract components
            if (mmcMeta.components) {
                const mcComp = mmcMeta.components.find((c: any) => c.uid === 'net.minecraft')
                if (mcComp && mcComp.version) {
                    selectedVersion.value = mcComp.version
                }
                
                // Detect loader from components
                const fabric = mmcMeta.components.find((c: any) => c.uid === 'net.fabricmc.fabric-loader')
                const quilt = mmcMeta.components.find((c: any) => c.uid === 'org.quiltmc.quilt-loader')
                const forge = mmcMeta.components.find((c: any) => c.uid === 'net.minecraftforge')
                const neoforge = mmcMeta.components.find((c: any) => c.uid === 'net.neoforged')
                
                if (fabric) modpackLoader.value = 'fabric'
                else if (quilt) modpackLoader.value = 'quilt'
                else if (neoforge) modpackLoader.value = 'neoforge'
                else if (forge) modpackLoader.value = 'forge'
            }
        } catch (e) {
            console.error('Failed to parse mmc-pack metadata (might be generic zip)', e)
        }
    }

    // Auto-advance logic
    serverName.value = customModpackName.value.replace(/\.(mrpack|zip)$/, '')
    selectedModpack.value = { title: customModpackName.value } // Fake modpack object for UI
    step.value++
  }
}

function goBack() {
  if (step.value > 1) {
    step.value--
  } else {
    navigateTo('/')
  }
}



async function detectJavaVersion() {
  checkingJava.value = true
  try {
     await scanJava()
     
     // Determine required java based on MC version
     const mcVersion = selectedVersion.value || ''
     let requiredJava = 17
     if (mcVersion >= '1.20.5') requiredJava = 21
     else if (mcVersion >= '1.17') requiredJava = 17
     else requiredJava = 8
     
     const compatible = getJavaForVersion(installations.value, requiredJava)
     
     if (compatible) {
         detectedJavaVersion.value = `Java ${compatible.major || '?'} (${compatible.version || 'unknown'})`
     } else {
         if (installations.value.length > 0) {
             const best = installations.value[0]
             if (best) {
                detectedJavaVersion.value = `Java ${best.major || '?'} (${best.version || 'unknown'})`
             }
         } else {
             detectedJavaVersion.value = 'No Java detected'
         }
     }
  } catch (e) {
     console.error('Failed to detect java', e)
     detectedJavaVersion.value = 'Detection Failed'
  } finally {
     checkingJava.value = false
  }
}

async function fetchVersions() {
  if (!selectedTypeData.value) return
  if (selectedType.value === 'modpack') return // Modpacks use searchModpacks instead
  
  isLoadingVersions.value = true
  availableVersions.value = []
  selectedVersion.value = undefined

  try {
    const apiConfig = selectedTypeData.value.api
    let url = apiConfig.url

    if (includeSnapshots.value && apiConfig.snapshotUrl) {
      url = apiConfig.snapshotUrl
    }

    const response = await fetch(url)
    const data = await response.json()

    if (apiConfig.type === 'papermc') {
      const versionsObj = data.versions || {}
      const allVersions: string[] = []
      
      Object.keys(versionsObj).forEach(majorVer => {
        if (Array.isArray(versionsObj[majorVer])) {
          allVersions.push(...versionsObj[majorVer])
        }
      })
      
      availableVersions.value = allVersions.sort((a, b) => b.localeCompare(a, undefined, { numeric: true }))
    } else {
      if (Array.isArray(data)) {
        availableVersions.value = data
      }
    }
  } catch (error) {
    console.error('Failed to fetch versions:', error)
  } finally {
    isLoadingVersions.value = false
  }
}

// Watchers
watch(step, (newStep) => {
  if (newStep === 2 && selectedType.value !== 'custom') {
    fetchVersions()
    includeSnapshots.value = false 
  }
  if (newStep === 4) {
    detectJavaVersion()
  }
})

watch(includeSnapshots, () => {
  if (step.value === 2) {
    fetchVersions()
  }
})

async function resolveDownloadUrl(): Promise<string | null> {
  if (!selectedTypeData.value || !selectedVersion.value) return null
  
  const apiConfig = selectedTypeData.value.api
  const version = selectedVersion.value

  try {
    if (apiConfig.type === 'papermc') {
      const url = `${apiConfig.url}/versions/${version}/builds/latest`
      const response = await fetch(url)
      const data = await response.json()
      
      if (data?.downloads?.['server:default']?.url) {
        return `https://fill-data.papermc.io/v1/objects/${data.downloads['server:default'].checksums.sha256}/${data.downloads['server:default'].name}`
      }
      return null
    } else {
      let baseUrl = apiConfig.url
      
      if (baseUrl.includes('get-versions')) {
        let downloadUrl = baseUrl.replace('get-versions', 'get-jar')
        
        if (includeSnapshots.value && apiConfig.snapshotUrl) {
          downloadUrl = apiConfig.snapshotUrl.replace('get-versions', 'get-jar')
        }
        
        return `${downloadUrl}/${version}`
      }
      
      return null
    }
  } catch (e) {
    console.error('Error resolving download URL', e)
    return null
  }
}

// Add new state for installed files
const installedModpackFiles = ref<any[]>([])

const installedModpackDependencies = ref<any>(null)
const installedModpackMetadata = ref<Record<string, any>>({})

async function createServer() {
  if (!isValid.value) return

  isCreating.value = true
  installedModpackFiles.value = []
  installedModpackDependencies.value = null
  installedModpackMetadata.value = {}
  
  try {
    const uniqueId = uuidv4().slice(0, 8)
    const safeName = (serverName.value || 'untitled')
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/(^-|-$)/g, '')
    const folderName = `${safeName}-${uniqueId}`
    const relativePath = `VoidLink/servers/${folderName}`

    statusMessage.value = 'Creating directory...'
    await mkdir(relativePath, { baseDir: BaseDirectory.Document, recursive: true })

    const jarName = 'server.jar'
    
    if (selectedType.value === 'custom') {
      if (!customJarPath.value) throw new Error('No JAR selected')
      
      statusMessage.value = 'Copying JAR file...'
      // Ensure absolute path for destination
      const { documentDir } = await import('@tauri-apps/api/path')
      const docDir = await documentDir()
      const destPath = await join(docDir, relativePath, jarName)
      
      await copyFile(customJarPath.value, destPath)
    } else if (selectedType.value === 'modpack') {
      if (customModpackPath.value) {
          // Install from local file
          creationProgress.value = 10
          statusMessage.value = 'Reading modpack file...'
          
          // Read file as binary
          const { readFile } = await import('@tauri-apps/plugin-fs')
          const fileData = await readFile(customModpackPath.value)
          
          if (customModpackPath.value.endsWith('.mrpack')) {
             statusMessage.value = 'Installing Modpack (.mrpack)...'
             // Ensure metadata is set if not already (redundant check)
             if (customModpackMetadata.value) {
                // We can use this to set loader info in server.json later if needed
             }
             
             const result = await installMrpack(fileData, relativePath, (msg, prog) => {
                 statusMessage.value = msg
                 if (prog !== undefined) creationProgress.value = prog
             })
             
             if (result && result.files) {
                installedModpackFiles.value = result.files
                installedModpackDependencies.value = result.dependencies
                installedModpackMetadata.value = result.metadata || {}
             }
          } else {
             statusMessage.value = 'Installing Zip (.zip)...'
             await installZip(fileData, relativePath, (msg, prog) => {
                 statusMessage.value = msg
                 if (prog !== undefined) creationProgress.value = prog
             })
          }
      } else {
          // Standard Modrinth Install
          const modpackData = selectedModpackVersionData.value
          if (!modpackData) throw new Error("No modpack version selected")
          
          const file = modpackData.files.find((f: any) => f.primary) || modpackData.files[0]
          if (!file) throw new Error("No file found for modpack version")
          
          statusMessage.value = 'Initializing installer...'
          const result = await installModpack(file.url, relativePath, (msg, prog) => {
              statusMessage.value = msg
              if (prog !== undefined) creationProgress.value = prog
          })
          
          if (result && result.files) {
            installedModpackFiles.value = result.files
            installedModpackDependencies.value = result.dependencies
            installedModpackMetadata.value = result.metadata || {}
          }
      }
    } else {
      statusMessage.value = 'Resolving download URL...'
      const downloadUrl = await resolveDownloadUrl()
      
      if (!downloadUrl) {
        statusMessage.value = 'Failed to resolve download URL.'
        isCreating.value = false
        return
      }

      statusMessage.value = 'Downloading server JAR...'
      const response = await fetch(downloadUrl)
      if (!response.ok) throw new Error(`Failed to download: ${response.statusText}`)
      
      const buffer = await response.arrayBuffer()
      await writeFile(
        await join(relativePath, jarName),
        new Uint8Array(buffer),
        { baseDir: BaseDirectory.Document }
      )
    }

    statusMessage.value = 'Writing server.properties...'
    
    // Create server.properties with world settings
    const serverProperties = [
      `level-seed=${worldSeed.value}`,
      `level-type=minecraft\\:${worldType.value}`,
      `generate-structures=${generateStructures.value}`,
      `hardcore=${hardcoreMode.value}`,
      `motd=\u00A73Server Powered by \u00A7bVoidLink`,
      `max-players=20`,
      `pvp=true`,
      `online-mode=true`,
      `gamemode=survival`,
      `difficulty=normal`,
      `server-port=25565`,
      `enable-command-block=true`,
      `enable-rcon=true`,
      `broadcast-rcon-to-ops=false`,
      `rcon.port=25575`,
      `rcon.password=voidlink`
    ].join('\n')

    await writeTextFile(
      await join(relativePath, 'server.properties'),
      serverProperties,
      { baseDir: BaseDirectory.Document }
    )

    statusMessage.value = 'Creating server configuration...'

    // Save extended modpack metadata
    if (selectedType.value === 'modpack' && installedModpackFiles.value.length > 0) {
        // Save addons.json with resolved metadata
        if (installedModpackMetadata.value && Object.keys(installedModpackMetadata.value).length > 0) {
           await writeTextFile(
              await join(relativePath, 'addons.json'),
              JSON.stringify(installedModpackMetadata.value, null, 2), 
              { baseDir: BaseDirectory.Document }
           )
        }
    }

    // Create Metadata
    const metadata = {
      id: uniqueId,
      name: serverName.value,
      type: selectedTypeData.value?.id,
      typeName: selectedTypeData.value?.name,
      modpack: selectedModpack.value ? {
         id: selectedModpack.value.id || 'custom',
         title: selectedModpack.value.title || customModpackName.value,
         versionId: selectedVersion.value || 'custom',
         versionName: selectedVersion.value || 'custom',
         slug: selectedModpack.value.slug || customModpackName.value,
         loader: modpackLoader.value,
         version: selectedType.value === 'custom' ? 'custom' : (selectedModpackVersionData.value?.id || 'custom'),
         // Store extended metadata
         files: installedModpackFiles.value,
         dependencies: installedModpackDependencies.value
      } : null,
      supportsPlugins: selectedTypeData.value?.id === 'custom' ? customSupportsPlugins.value : selectedTypeData.value?.supportsPlugins,
      supportsMods: selectedTypeData.value?.id === 'custom' ? customSupportsMods.value : selectedTypeData.value?.supportsMods,
      version: selectedVersion.value || 'Custom',
      icon: selectedTypeData.value?.icon,
      jarFile: jarName,
      createdAt: new Date().toISOString(),
      path: relativePath,
      javaSettings: {
        memory: ramLimit.value,
        path: 'java',
        flags: ''
      }
    }

    await writeTextFile(
      await join(relativePath, 'server.json'),
      JSON.stringify(metadata, null, 2),
      { baseDir: BaseDirectory.Document }
    )

    statusMessage.value = 'Done! Launching server...'
    await new Promise(resolve => setTimeout(resolve, 500))
    
    // Refresh servers list in sidebar
    const serversStore = useServersStore()
    await serversStore.refreshServers()
    
    // Navigate to the new server
    navigateTo(`/server/${folderName}`)

  } catch (error) {
    console.error('Creation failed', error)
    statusMessage.value = `Error: ${error}`
  } finally {
    isCreating.value = false
  }
}
</script>

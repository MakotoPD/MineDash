<template>
	<UDashboardGroup unit="rem" class="bg-[#101922]">
		<UDashboardSidebar 
			resizable 
			collapsible
			:min-size="12"
			:default-size="15"
			:max-size="15"
			:collapsed-size="0"
			v-model:collapsed="collapsed"
			:ui="{ footer: 'border-t border-gray-950', body: 'px-3' }"
		>
			<template #resize-handle="{ onMouseDown, onTouchStart, onDoubleClick }">
				<UDashboardResizeHandle
				class="after:absolute after:inset-y-0 after:right-0 after:w-px hover:after:bg-primary-800 after:transition"
				@mousedown="onMouseDown"
				@touchstart="onTouchStart"
				@dblclick="onDoubleClick"
				/>
			</template>
			<div class="w-full">
				<div class="flex justify-center items-center mt-1 gap-2">
					<img src="/icon.png" alt="icon" width="36" height="36">
					<p>MineDash</p>
				</div>
				


				<div class="my-8">
					<p class="text-xs text-gray-500">MAIN</p>
					<div class="flex flex-col mt-1">
						<UButton class="cursor-pointer" color="primary" variant="ghost" icon="i-lucide-home" to="/">Dashboard</UButton>
						<UButton class="cursor-pointer" color="primary" variant="ghost" icon="i-lucide-settings" to="/settings">Settings</UButton>
					</div>
				</div>

				<div class="px-1 flex flex-col gap-2 w-full">
					<div class="w-full flex items-center justify-between">
						<p class="text-xs text-gray-500">SERVERS</p>
						<UButton class="cursor-pointer" size="xs" color="primary" variant="ghost" icon="i-lucide-plus" to="/create"></UButton>
					</div>
					<div
					v-for="server in servers"
					:key="server.id"
					class="group cursor-pointer w-full p-2 hover:bg-neutral-800 border border-[#101922] hover:border-neutral-700 rounded-lg transition-all"
					@click="navigateTo(`/server/${server.path.split('/').pop()}`)"
					>
						<div class="flex items-center justify-start">
							<div class="flex items-center gap-4">
								<div>
									<UChip 
										:color="getStatusColor(getServerStatus(server.path.split('/').pop() || ''))"
										position="bottom-right"
									>
										<div class="w-8 h-8 rounded-lg bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center">
											<UIcon :name="server.icon || 'i-lucide-box'" class="w-6 h-6 text-primary-600 dark:text-primary-400" />
										</div>
									</UChip>
									
								</div>

								<div>
									<h3 class="text-gray-900 dark:text-white text-sm">{{ server.name }}</h3>
									<p :class="getStatusColor(getServerStatus(server.path.split('/').pop() || '')) === 'error' ? 'text-gray-500' : 'text-'+getStatusColor(getServerStatus(server.path.split('/').pop() || ''))" class="text-xs">{{ getServerStatus(server.path.split('/').pop() || '') }}</p>
								</div>
							</div>
						</div>
					</div>
				</div>

			</div>
		</UDashboardSidebar>
		<div class="flex-1 overflow-y-auto bg-[#0b1116]">
			<slot />
		</div>
	</UDashboardGroup>
</template>

<script lang="ts" setup>
import { useServersStore } from '~/stores/useServersStore'
import { storeToRefs } from 'pinia'

const collapsed = ref(false)

// Use centralized servers store
const serversStore = useServersStore()
const { servers, loading } = storeToRefs(serversStore)
const { loadServers, getServerStatus, getStatusColor } = serversStore

onMounted(async () => {
  await loadServers()
})
</script>
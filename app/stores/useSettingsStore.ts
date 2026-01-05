import { defineStore } from 'pinia'
import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs'

interface JavaInstallations {
	java8: string
	java11: string
	java17: string
	java21: string
}

export interface Settings {
	minimizeOnClose: boolean
	javaPath: string
	defaultMemory: number
	defaultFlags: string
	javaInstallations: JavaInstallations
}

export const useSettingsStore = defineStore('settings', () => {
	const settings = reactive<Settings>({
		minimizeOnClose: true, // Default to TRUE (Minimize) per requirements
		javaPath: 'java',
		defaultMemory: 4,
		defaultFlags: '',
		javaInstallations: {
			java8: '',
			java11: '',
			java17: '',
			java21: ''
		}
	})

	const loaded = ref(false)
	const systemRamGB = ref(32)

	async function loadSettings() {
		if (loaded.value) return // Don't reload unnecessarily

		try {
			const content = await readTextFile('MineDash/settings.json', { baseDir: BaseDirectory.Document })
			const data = JSON.parse(content)
			if (data) {
				settings.minimizeOnClose = data.minimizeOnClose ?? true
				settings.javaPath = data.javaPath ?? 'java'
				settings.defaultMemory = data.defaultMemory ?? 4
				settings.defaultFlags = data.defaultFlags ?? ''

				if (data.javaInstallations) {
					settings.javaInstallations = {
						java8: data.javaInstallations.java8 ?? '',
						java11: data.javaInstallations.java11 ?? '',
						java17: data.javaInstallations.java17 ?? '',
						java21: data.javaInstallations.java21 ?? ''
					}
				}
			}
		} catch (e) {
			console.log('No settings file found or failed to read, using defaults.')
		} finally {
			loaded.value = true
		}

		// Also fetch system RAM
		try {
			// Dynamic import to avoid SSR issues if this runs in server context? Tauri is client only though.
			const { invoke } = await import('@tauri-apps/api/core')
			const sysInfo = await invoke<{ total_memory_bytes: number }>('get_system_info')
			systemRamGB.value = Math.floor(sysInfo.total_memory_bytes / (1024 * 1024 * 1024))
		} catch (e) {
			console.log('Failed to get system info')
		}
	}

	async function saveSettings() {
		try {
			await writeTextFile(
				'MineDash/settings.json',
				JSON.stringify(settings, null, 2),
				{ baseDir: BaseDirectory.Document }
			)
			console.log('Settings saved automatically')
		} catch (e) {
			console.error('Failed to save settings:', e)
		}
	}

	// Auto-save whenever settings change
	watch(() => settings, () => {
		saveSettings()
	}, { deep: true })

	return {
		settings,
		loaded,
		systemRamGB,
		loadSettings,
		saveSettings
	}
})

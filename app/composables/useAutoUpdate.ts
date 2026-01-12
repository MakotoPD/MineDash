import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

export const useAutoUpdate = () => {
	const toast = useToast()

	const checkForUpdates = async (silent = false) => {
		console.log('[AutoUpdate] Checking for updates...', { silent })
		try {
			const update = await check()
			console.log('[AutoUpdate] Update check result:', update)

			if (update) {
				console.log(`[AutoUpdate] Update available: ${update.version}`)
				toast.add({
					title: 'Update Available',
					description: `Version ${update.version} is available.`,
					icon: 'i-lucide-download',
					color: 'primary',
					duration: 0,
					actions: [{
						label: 'Update & Restart',
						onClick: async () => {
							console.log('[AutoUpdate] Starting download and install...')
							let downloaded = 0
							let contentLength = 0

							toast.add({
								title: 'Downloading Update',
								description: 'Please wait...',
								icon: 'i-lucide-loader-2',
								color: 'primary',
								duration: 0,
								// @ts-expect-error - loading prop exists on some versions/custom toasts but standard uses icon
								loading: true
							})

							await update.downloadAndInstall((event) => {
								console.log('[AutoUpdate] Install event:', event.event, event.data)
								switch (event.event) {
									case 'Started':
										contentLength = event.data.contentLength || 0
										break
									case 'Progress':
										downloaded += event.data.chunkLength
										break
									case 'Finished':
										console.log('[AutoUpdate] Download finished, relaunching...')
										break
								}
							})

							await relaunch()
						}
					}, {
						label: 'Later',
						onClick: () => { console.log('[AutoUpdate] User dismissed update') }
					}]
				})
			} else if (!silent) {
				console.log('[AutoUpdate] No update found (up to date)')
				toast.add({
					title: 'Up to date',
					description: 'You are running the latest version.',
					icon: 'i-lucide-check-circle',
					color: 'success'
				})
			}
		} catch (error) {
			console.error('[AutoUpdate] Failed to check for updates:', error)
			if (!silent) {
				toast.add({
					title: 'Update Check Failed',
					description: String(error),
					icon: 'i-lucide-alert-circle',
					color: 'error'
				})
			}
		}
	}

	return {
		checkForUpdates
	}
}

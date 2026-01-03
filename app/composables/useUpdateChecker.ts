import { fetch } from '@tauri-apps/plugin-http'
import { APP_VERSION, GITHUB_REPO, GITHUB_RELEASES_URL } from '~/utils/version'

export interface UpdateInfo {
	available: boolean
	currentVersion: string
	latestVersion: string
	releaseUrl: string
	releaseNotes: string
	publishedAt: string
}

export function useUpdateChecker() {
	const updateInfo = ref<UpdateInfo | null>(null)
	const checking = ref(false)
	const error = ref<string | null>(null)

	async function checkForUpdates(): Promise<UpdateInfo> {
		checking.value = true
		error.value = null

		try {
			const res = await fetch(`https://api.github.com/repos/${GITHUB_REPO}/releases/latest`, {
				headers: {
					'Accept': 'application/vnd.github.v3+json',
					'User-Agent': 'MineDash-UpdateChecker'
				}
			})

			if (!res.ok) {
				throw new Error(`GitHub API error: ${res.status}`)
			}

			const release = await res.json()
			const latestVersion = release.tag_name.replace(/^v/, '') // Remove 'v' prefix if present

			const info: UpdateInfo = {
				available: compareVersions(latestVersion, APP_VERSION) > 0,
				currentVersion: APP_VERSION,
				latestVersion,
				releaseUrl: release.html_url || GITHUB_RELEASES_URL,
				releaseNotes: release.body || '',
				publishedAt: release.published_at || ''
			}

			updateInfo.value = info
			return info

		} catch (e) {
			error.value = e instanceof Error ? e.message : 'Failed to check for updates'
			console.error('Update check failed:', e)

			return {
				available: false,
				currentVersion: APP_VERSION,
				latestVersion: APP_VERSION,
				releaseUrl: GITHUB_RELEASES_URL,
				releaseNotes: '',
				publishedAt: ''
			}
		} finally {
			checking.value = false
		}
	}

	// Compare semantic versions: returns 1 if a > b, -1 if a < b, 0 if equal
	function compareVersions(a: string, b: string): number {
		const partsA = a.split('.').map(n => parseInt(n, 10) || 0)
		const partsB = b.split('.').map(n => parseInt(n, 10) || 0)

		for (let i = 0; i < Math.max(partsA.length, partsB.length); i++) {
			const numA = partsA[i] || 0
			const numB = partsB[i] || 0
			if (numA > numB) return 1
			if (numA < numB) return -1
		}
		return 0
	}

	return {
		updateInfo,
		checking,
		error,
		checkForUpdates,
		currentVersion: APP_VERSION,
		releasesUrl: GITHUB_RELEASES_URL
	}
}

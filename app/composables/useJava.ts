import { invoke } from '@tauri-apps/api/core'

export interface JavaInstallation {
	path: string
	version?: string
	major?: number
	vendor?: string
	arch?: string
	is_valid: boolean
}

export interface JavaValidation {
	is_valid: boolean
	version?: string
	major?: number
	vendor?: string
	arch?: string
	error?: string
}

export interface AdoptiumRelease {
	version: string
	major: number
	download_url: string
	filename: string
	size: number
	checksum: string | null
}

export const useJava = () => {
	const installations = ref<JavaInstallation[]>([])
	const loading = ref(false)
	const error = ref<string | null>(null)

	const scanJava = async () => {
		loading.value = true
		error.value = null
		try {
			installations.value = await invoke<JavaInstallation[]>('detect_java_installations_cmd')
		} catch (e) {
			console.error('Failed to scan for Java:', e)
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	const validateJavaPath = async (path: string): Promise<JavaValidation> => {
		try {
			return await invoke<JavaValidation>('validate_java_path_cmd', { path })
		} catch (e) {
			console.error('Failed to validate Java path:', e)
			return {
				is_valid: false,
				error: String(e)
			}
		}
	}

	const fetchAdoptiumRelease = async (major: number): Promise<AdoptiumRelease | null> => {
		try {
			return await invoke<AdoptiumRelease>('fetch_adoptium_release_cmd', { major })
		} catch (e) {
			console.error(`Failed to fetch Adoptium release for Java ${major}:`, e)
			return null
		}
	}

	const downloadJava = async (major: number, installDir: string): Promise<string> => {
		return await invoke<string>('download_java_cmd', { major, installDir })
	}

	/**
	 * Returns the best matching Java installation for a given major version.
	 * If no exact match is found, returns the highest version available if allowable,
	 * or undefined.
	 */
	const getJavaForVersion = (entries: JavaInstallation[], requiredMajor: number): JavaInstallation | undefined => {
		// 1. Try to find exact major match with validation
		const exact = entries.find(j => j.major === requiredMajor && j.is_valid)
		if (exact) return exact

		// 2. If we need Java 8, we strictly need Java 8 usually (though newer *can* run it, older MCs are picky)
		// But generally for MC, newer Java runs newer MC. 
		// If we need Java 17, Java 21 is usually fine.
		// If we need Java 21, Java 17 is NOT fine.

		// So look for any valid java >= requiredMajor, sorted by version ascending (closest match)
		const compatible = entries
			.filter(j => j.is_valid && (j.major || 0) >= requiredMajor)
			.sort((a, b) => (a.major || 0) - (b.major || 0))

		return compatible[0]
	}

	return {
		installations,
		loading,
		error,
		scanJava,
		validateJavaPath,
		fetchAdoptiumRelease,
		downloadJava,
		getJavaForVersion
	}
}

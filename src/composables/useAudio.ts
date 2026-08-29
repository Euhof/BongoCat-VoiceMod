import type { UnlistenFn } from '@tauri-apps/api/event'

import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onUnmounted, ref } from 'vue'

export function useAudio() {
  const volume = ref(0)
  const isCapturing = ref(false)

  let unlisten: UnlistenFn | null = null
  let smoothed = 0

  // 0.25 = mais suave | 0.45 = mais responsivo
  const SMOOTHING = 0.32

  const start = async () => {
    if (isCapturing.value) return

    try {
      unlisten = await listen<number>('audio-volume', (event) => {
        const raw = event.payload
        smoothed = smoothed * (1 - SMOOTHING) + raw * SMOOTHING
        volume.value = smoothed
      })

      await invoke('start_audio_capture')
      isCapturing.value = true
    } catch (err) {
      console.error('[useAudio] Falha ao iniciar captura:', err)
    }
  }

  const stop = async () => {
    if (!isCapturing.value) return

    try {
      if (unlisten) {
        unlisten()
        unlisten = null
      }

      await invoke('stop_audio_capture')
      isCapturing.value = false
      volume.value = 0
      smoothed = 0
    } catch (err) {
      console.error('[useAudio] Falha ao parar captura:', err)
    }
  }

  onUnmounted(() => {
    stop()
  })

  return {
    volume,
    isCapturing,
    start,
    stop,
  }
}

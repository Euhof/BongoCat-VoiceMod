import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { onUnmounted, ref } from 'vue'

export function useAudio() {
  const volume = ref(0)
  const isCapturing = ref(false)
  let unlisten: (() => void) | null = null

  // Suavização (evita boca tremendo)
  let smoothed = 0
  const smoothing = 0.35 // 0.2 = mais suave | 0.5 = mais responsivo

  const start = async () => {
    if (isCapturing.value) return

    try {
      await invoke('start_audio_capture')
      isCapturing.value = true

      unlisten = await listen<number>('audio-volume', (event) => {
        const raw = event.payload
        smoothed = smoothed * (1 - smoothing) + raw * smoothing
        volume.value = smoothed
      })
    } catch (err) {
      console.error('Erro ao iniciar captura de áudio:', err)
    }
  }

  const stop = async () => {
    if (!isCapturing.value) return

    try {
      await invoke('stop_audio_capture')
      isCapturing.value = false
      volume.value = 0
      smoothed = 0

      if (unlisten) {
        unlisten()
        unlisten = null
      }
    } catch (err) {
      console.error('Erro ao parar captura de áudio:', err)
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

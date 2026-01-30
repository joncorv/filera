import { ref, onMounted, onUnmounted } from 'vue'

interface Position {
  x: number
  y: number
}

interface NoiseOptions {
  size?: number      // Scale of the noise pattern (default: 1)
  speed?: number     // How fast the noise evolves (default: 0.001)
  amplitude?: number // Max displacement in pixels (default: 100)
}

export function usePosNoise(options: NoiseOptions = {}) {
  const { size = 1, speed = 0.001, amplitude = 100 } = options
  const position = ref<Position>({ x: 0, y: 0 })
  let startTime = Date.now()
  let animationId: number

  const update = () => {
    const t = (Date.now() - startTime) * speed
    
    // Generate 2D noise using offset sine waves
    const noiseX = (
      Math.sin(t / size) + 
      Math.sin(t * 2.3 / size) * 0.5 + 
      Math.sin(t * 0.7 / size) * 0.3
    ) / 1.8 // Normalize to roughly -1 to 1
    
    const noiseY = (
      Math.sin(t * 1.1 / size + 100) + 
      Math.sin(t * 1.9 / size + 100) * 0.5 + 
      Math.sin(t * 0.8 / size + 100) * 0.3
    ) / 1.8
    
    position.value = {
      x: noiseX * amplitude,
      y: noiseY * amplitude
    }
    
    animationId = requestAnimationFrame(update)
  }

  onMounted(() => update())
  onUnmounted(() => cancelAnimationFrame(animationId))

  return position
}
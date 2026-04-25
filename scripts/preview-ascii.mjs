import { generateAsciiFrames } from '../engine/ascii-renderer/animator.ts'
import { getPetTemplate, renderPetAnsi } from '../engine/ascii-renderer/generator.ts'

const args = new Map()
for (const arg of process.argv.slice(2)) {
  const [key, value = 'true'] = arg.replace(/^--/, '').split('=')
  args.set(key, value)
}

const dna = args.get('dna') ?? 'oso-preview-seed'
const tier = Number(args.get('tier') ?? '5')
const mode = args.get('mode') ?? 'idle'
const animate = args.get('animate') === 'true'
const personality = {
  curiosity: Number(args.get('curiosity') ?? '0.71'),
  boldness: Number(args.get('boldness') ?? '0.48'),
  empathy: Number(args.get('empathy') ?? '0.66'),
}

const template = getPetTemplate(dna, tier)

if (!animate) {
  process.stdout.write(`Variant: ${template.name}\n`)
  process.stdout.write(renderPetAnsi(dna, tier, personality))
  process.stdout.write('\n')
  process.exit(0)
}

const frames = generateAsciiFrames(dna, tier, personality, mode)
let index = 0
process.stdout.write(`Variant: ${template.name} (${mode})\n`)

const interval = setInterval(() => {
  process.stdout.write('\u001bc')
  process.stdout.write(`Variant: ${template.name} (${mode})\n`)
  process.stdout.write(`${frames[index % frames.length]}\n`)
  index += 1
}, mode === 'evolving' ? 180 : 260)

setTimeout(() => {
  clearInterval(interval)
  process.exit(0)
}, 3200)

export type TemplateTier = 0 | 1 | 2 | 3 | 4 | 5

export type PetTemplate = {
  name: string
  lines: string[]
  maskFixed?: boolean
}

export const OMO_TEMPLATES: Record<TemplateTier, PetTemplate[]> = {
  0: [
    { name: 'Seed', lines: ['    {auraL}', '   ( {eyes} )', '    {mouth}'] },
    { name: 'Blank', lines: ['    . .', '    {eyes}', '    {mouth}'] },
    { name: 'Mist', lines: ['   {auraL} {auraR}', '   · {eyes} ·', '    {mouth}'] },
  ],
  1: [
    { name: 'Wisp', lines: ['   ( {eyes} )', '    {mouth}'] },
    { name: 'Nub', lines: ['   .--.', '  ( {eyes} )', '   {mouth}'] },
    { name: 'Pebble', lines: ['   ╭───╮', '  ( {eyes} )', '   ╰{base}╯'] },
  ],
  2: [
    { name: 'Spark', lines: ['   /\\_/\\', '  ( {eyes} )', '   ^ {mouth} ^'] },
    { name: 'Floof', lines: ['   ({eyes})', '    {mouth}'] },
    { name: 'Zig', lines: ['   {auraL}{auraR}', '  ( {eyes} )', '   {auraL}{auraR}'] },
    { name: 'Moss', lines: ['   /===/\\', '  ( {eyes} )', '   | {mouth} |'] },
  ],
  3: [
    { name: 'Stargazer', lines: ['   {sigilL} {sigilR}', '  ( {eyes} )', '   {base}'] },
    { name: 'Ember', lines: ['   /───\\', '  ( {eyes} )', '   \\{base}/'] },
    { name: 'Void', lines: ['   /────\\', '  ( {eyes} )', '   \\{base}/'] },
    { name: 'Luna', lines: ['   {auraL} /\\', '  ( {eyes} )', '   {auraR} \\/'] },
  ],
  4: [
    { name: 'Crown', lines: ['   {auraL}╭━✦━╮{auraR}', '    ( {eyes} )', '     ╰{base}╯'] },
    { name: 'Aether', lines: ['   /╭─╮\\', '  ( {eyes} )', '   \\╰─╯/'] },
    { name: 'Storm', lines: ['   ⚡⚡', '  ( {eyes} )', '   ⚡⚡'] },
    { name: 'Crystal', lines: ['   💎💎', '  ( {eyes} )', '   💎💎'] },
  ],
  5: [
    { name: 'Ọ̀ṣỌ́ Prime', maskFixed: true, lines: ['   /\\_/\\', '  ({mask})', '   > {mouth} <'] },
    { name: 'Ọ̀ṣỌ́ Sovereign', maskFixed: true, lines: ['   ╭━━━━━╮', '  / {mask} \\', ' (   {mouth}   )', '  ╰━━━━━╯'] },
    { name: 'Ọ̀ṣỌ́ Eclipse', maskFixed: true, lines: ['   ☾☽☾', '  /{mask}\\', ' (  {mouth}  )', '  \\___/'] },
    { name: 'Ọ̀ṣỌ́ Emberlord', maskFixed: true, lines: ['   /===/\\', '  | {mask} |', '   | {mouth} |'] },
    { name: 'Ọ̀ṣỌ́ Starforged', maskFixed: true, lines: ['   ✨✨✨', '  / {mask} \\', ' (   {sigilL}   )', '  \\_____/'] },
    { name: 'Ọ̀ṣỌ́ Voidking', maskFixed: true, lines: ['   ╭─────╮', '  /  {mask}  \\', ' (   {mouth}   )', '  ╰─────╯'] },
    { name: 'Ọ̀ṣỌ́ Bloom', maskFixed: true, lines: ['   🌸 🌸', '  / {mask} \\', ' (   {mouth}   )', '  \\_____/'] },
    { name: 'Ọ̀ṣỌ́ Thunder', maskFixed: true, lines: ['   ⚡⚡', '  /{mask}\\', ' (  {mouth}  )', '  \\___/'] },
    { name: 'Ọ̀ṣỌ́ Moon', maskFixed: true, lines: ['   ☽ ☾', '  /{mask}\\', ' (  {mouth}  )', '  \\___/'] },
    { name: 'Ọ̀ṣỌ́ Flame', maskFixed: true, lines: ['   🔥🔥', '  /{mask}\\', ' (  {mouth}  )', '  \\___/'] },
    { name: 'Ọ̀ṣỌ́ Crystal', maskFixed: true, lines: ['   💎💎', '  /{mask}\\', ' (  {sigilR}  )', '  \\___/'] },
    { name: 'Ọ̀ṣỌ́ Eternal', maskFixed: true, lines: ['   ╭━━━━━╮', '  /  {mask}  \\', ' (  {eyes}  )', '  ╰━━━━━╯'] },
    { name: 'Ọ̀ṣỌ́ Legend', maskFixed: true, lines: ['   ✨✨✨', '  / {mask} \\', ' (   ∞   )', '  \\_____/'] },
  ],
}

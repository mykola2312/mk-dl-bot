import type { BotOptions } from 'gramio'
import { Bot } from 'gramio'
import { ytdlp_info } from './dl/ytdlp.ts'

const bot = new Bot({
  token: Deno.env.get('BOT_TOKEN'),
  api: {
    baseURL: Deno.env.get('BOT_API_URL'),
  },
} as BotOptions)

bot.command('test', async (ctx) => {
  await ytdlp_info('')

  ctx.send('ok')
})

bot.start()

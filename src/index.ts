import type { BotOptions } from 'gramio'
import { Bot } from 'gramio'

const bot = new Bot({
  token: Deno.env.get('BOT_TOKEN'),
  api: {
    baseURL: Deno.env.get('BOT_API_URL'),
  },
} as BotOptions)

bot.command('test', (ctx) => {
  ctx.send('yooo wassup')
})

bot.start()

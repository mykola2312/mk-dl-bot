import type { BotOptions } from 'gramio'
import { Bot } from 'gramio'
import { spawn } from './spawn.ts'

const bot = new Bot({
  token: Deno.env.get('BOT_TOKEN'),
  api: {
    baseURL: Deno.env.get('BOT_API_URL'),
  },
} as BotOptions)

bot.command('test', async (ctx) => {
  // TEST
  try {
    await spawn('awdawda', [])
  }
  catch (e) {
    console.log(e)
  }

  await ctx.send('ok')
})

bot.start()

import { Bot } from "gramio"

const bot = new Bot(process.env.TOKEN as string)
    .command("start", (context) => context.send("Hi!"))
    .onStart(({ info }) => console.log(`✨ Bot ${info.username} was started!`));

bot.start();
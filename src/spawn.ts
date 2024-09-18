export interface SpawnOutput {
  status: number
  stdout: string
  stderr: string
}

export async function spawn(cmd: string, args: string[]): Promise<SpawnOutput> {
  const p = await new Deno.Command(cmd, {
    args,
    stdout: 'piped',
    stderr: 'piped',
  }).spawn()
}
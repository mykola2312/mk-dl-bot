// so I want wrapper for process spawning. It takes argument list (including program name),
// then spawns process and waits for completion. If status code is non-zero (or any other
// code that is considered failure), it will throw custom Error type that includes stderr
// piped output parsed into text. Otherwise, on considered success, it will return exectution
// to caller.

export interface SpawnOutput {
  code: number
  stdout: string
  stderr: string
}

export async function spawn(cmd: string, args: string[]): Promise<SpawnOutput> {
  const p = await new Deno.Command(cmd, {
    args,
    stdout: 'piped',
    stderr: 'piped',
  }).spawn()

  const { code, stdout, stderr } = await p.output()
  return {
    code,
    stdout: new TextDecoder().decode(stdout),
    stderr: new TextDecoder().decode(stderr),
  } as SpawnOutput
}

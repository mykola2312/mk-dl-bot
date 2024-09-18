export interface YtDlpFormat {
  format_id: string
  format_note?: string
  audio_channels?: number
  width?: number
  height?: number
  ext: string
  vcodec?: string
  acodec?: string
  vbr?: number
  abr?: number
}

export interface YtDlpInfo {
  id: string
  formats: YtDlpFormat[]
}

export async function ytdlp_info(url: string): Promise<void> {
  const p = await new Deno.Command('yt-dlp', {
    args: [url, '-j'],
    stdout: 'piped',
    stderr: 'piped',
  }).spawn()

  const [status, stdout] = await Promise.all([p.status, p.stdout])

  console.log(status)
  console.log(stdout)
}

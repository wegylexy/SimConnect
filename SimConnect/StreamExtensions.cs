namespace FlyByWireless.IO;

static class StreamExtensions
{
    public static async ValueTask ReadFullyAsync(this Stream stream, Memory<byte> buffer, CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        using var r = cancellationToken.Register(() => { using (stream) { } });
        while (!buffer.IsEmpty)
        {
            var read = await stream.ReadAsync(buffer, cancellationToken).ConfigureAwait(false);
            if (read == 0)
                throw new EndOfStreamException();
            buffer = buffer[read..];
        }
    }
}
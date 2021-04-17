using System;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

namespace FlyByWireless.IO
{
    static class StreamExtensions
    {
        public static async ValueTask ReadFullyAsync(this Stream stream, Memory<byte> buffer, CancellationToken cancellationToken)
        {
            cancellationToken.ThrowIfCancellationRequested();
            while (!buffer.IsEmpty)
            {
                var read = await stream.ReadAsync(buffer, cancellationToken).ConfigureAwait(false);
                if (read == 0)
                    throw new EndOfStreamException();
                buffer = buffer[read..];
            }
        }
    }
}
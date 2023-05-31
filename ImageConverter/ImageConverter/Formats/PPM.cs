using SkiaSharp;

namespace ImageConverter.Formats
{
    internal class PPM : IBitMapConverter
    {
        public PPM(string file)
        {
            File = file;
        }

        public string File { get; }

        public SKBitmap ToBitMap()
        {
            var (width, height, pixels) = GetSections();

            var image = new SKBitmap(width, height);
            using var canvas = new SKCanvas(image);
            using var paint = new SKPaint();
            paint.Style = SKPaintStyle.Stroke;

            for (int x = 0; x < width; x++)
            {
                for (int y = 0; y < height; y++)
                {
                    var color = pixels[Math.Clamp(x * width + y, 0, pixels.Count - 1)];
                    paint.Color = new SKColor(color[0], color[1], color[2]);
                    canvas.DrawPoint(y, x, paint);
                }
            }

            return image;
        }

        private (int width, int height, List<List<byte>> pixels) GetSections()
        {
            var pixelColors = File.LazySplit('\n').Select(x => x.Split(' ').Select(y => Convert.ToByte(y)).ToList());

            return (512, 512, pixelColors.ToList());
        }
    }
}

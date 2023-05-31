using ImageConverter.Formats;
using SkiaSharp;

namespace ImageConverter
{
    public class Conversion
    {
        public static void ToPng(string file, string path) 
        {
            var extension = GetExtension(path);

            var converter = extension switch
            {
                ".ppm" => new PPM(file),
                _ => throw new NotSupportedException(),
            };

            var bitmap = converter.ToBitMap();

            using var skImage = SKImage.FromBitmap(bitmap);
            using var data = skImage.Encode();
            using var stream = File.OpenWrite("..//..//..//output.png");

            data.SaveTo(stream);
        }

        private static string GetExtension(string file)
        {
            return file[file.LastIndexOf('.')..];
        }
    }
}

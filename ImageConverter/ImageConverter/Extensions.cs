namespace ImageConverter
{
    internal static class Extensions
    {
        public static IEnumerable<string> LazySplit(this string input, char separator)
        {
            int startIndex = 0;
            int separatorIndex = input.IndexOf(separator, startIndex);

            while (separatorIndex != -1)
            {
                yield return input[startIndex..separatorIndex];
                startIndex = separatorIndex + 1;
                separatorIndex = input.IndexOf(separator, startIndex);
            }

            yield return input[startIndex..];
        }

        public static IEnumerable<T[]> Batches<T>(this IEnumerable<T> input, int batchSize)
        {
            var enumerator = input.GetEnumerator();

            while (enumerator.MoveNext())
            {
                var batch = new T[batchSize];
                for (int i = 0; i < batchSize; i++)
                {
                    batch[i] = enumerator.Current;
                    if (!enumerator.MoveNext())
                    {
                        break;
                    }
                }

                yield return batch;
            }
        }
    }
}

// frontend/src/utils/contentProcessor.ts

/**
 * Cleans redundant titles from the beginning of LLM-generated content.
 * This is useful when the LLM includes the section title in its response.
 * 
 * @param outlineNodeTitle The title from the plan/outline (e.g., "1.2.2 Avoiding Pitfalls").
 * @param llmGeneratedContent The full content returned by the LLM.
 * @returns The content with the redundant title removed, if found.
 */
export function cleanRedundantTitle(outlineNodeTitle: string | undefined, llmGeneratedContent: string | null): string {
  if (!outlineNodeTitle || !llmGeneratedContent) {
    return llmGeneratedContent || '';
  }

  const lines = llmGeneratedContent.split('\n');
  const firstLine = lines[0].trim();

  if (!firstLine) {
    return llmGeneratedContent;
  }

  // Normalize both titles for a more robust comparison.
  // This handles minor differences in punctuation, spacing, and case.
  const normalize = (str: string) => 
    str.toLowerCase().replace(/[^\w\s]/g, '').replace(/\s+/g, ' ').trim();

  const normalizedOutlineTitle = normalize(outlineNodeTitle);
  const normalizedFirstLine = normalize(firstLine);

  // Check if the first line of the content is essentially a repeat of the outline title.
  // We check if the first line *starts with* the outline title to account for cases
  // where the LLM slightly elaborates on the title (e.g., "1.2.2 Avoiding Pitfalls" vs "1.2.2: Avoiding Common Pitfalls and Scams").
  if (normalizedFirstLine.startsWith(normalizedOutlineTitle)) {
    // If it's a match, return the content starting from the second line.
    return lines.slice(1).join('\n').trim();
  }

  // If no match, return the original content.
  return llmGeneratedContent;
}
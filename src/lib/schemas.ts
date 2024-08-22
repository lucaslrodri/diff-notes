import { z } from 'zod';

// Stack
export const stackStateSchema = z.object({
    left: z.string(),
    right: z.string(),
    side: z.string(),
    caret: z.number()
});
export const stackCurrentSchema = z.object({
    state: stackStateSchema,
    first: z.boolean(),
    last: z.boolean(),
});

export type StackStateSchema = z.infer<typeof stackStateSchema>;
export type StackCurrentSchema = z.infer<typeof stackCurrentSchema>;

export type LayoutSchema = "horizontal" | "vertical";
export type SideSchema = "left" | "right";

export const contentSchema = z.object({ left: z.string(), right: z.string() });
export type ContentSchema = z.infer<typeof contentSchema>;

// Highlight
const chunkSchema = z.object({ content: z.string(), op: z.string(), start: z.number(), end: z.number() })

export const textsChangesHighlightsSchema = z.object({
    left: z.array(chunkSchema),
    right: z.array(chunkSchema),
    raw: z.array(z.array(z.string()))
})
export type TextsChangesHighlightsSchema = z.infer<typeof textsChangesHighlightsSchema>;
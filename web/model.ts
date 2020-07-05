export namespace Model {
    export type Post = {
        id: number,
        body: string,
        posted_at: Date,
        updated_at: Date | null,
    };
}
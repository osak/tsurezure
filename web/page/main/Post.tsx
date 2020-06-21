import * as React from 'react';
import { Model } from '../../model';

export type Props = {
    post: Model.Post
};

export function Post(props: Props) {
    const post = props.post;

    const anchor = `post-${post.id}`;
    return <article className="article">
        <header className="header"><a id={anchor} href={`#${anchor}`}>■</a></header>
        <section className="body">{post.body}</section>
        <div className="posted-at">{post.posted_at}</div>
    </article>;
}
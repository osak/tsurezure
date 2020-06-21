import * as React from 'react';
import moment = require('moment');
import { Model } from '../../model';

export type Props = {
    post: Model.Post
};

export function Post(props: Props) {
    const post = props.post;

    const anchor = `post-${post.id}`;
    const postedAt = moment(post.posted_at).format('YYYY-MM-DD HH:mm:ssZ');
    return <article className="article">
        <header className="header"><a id={anchor} href={`#${anchor}`}>■</a></header>
        <section className="body">{post.body}</section>
        <div className="posted-at">{postedAt}</div>
    </article>;
}
import * as React from 'react';
import { Model } from '../../model';

export type Props = {
    post: Model.Post
};

export function Post(props: Props) {
    const post = props.post;

    return <div>
        <div><a id={`post-${post.id}`}>■</a></div>
        <div>{post.body}</div>
        <div>{post.posted_at}</div>
    </div>;
}
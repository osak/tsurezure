import * as React from 'react';
import moment = require('moment');
import { Model } from '../model';
import { Link } from '@reach/router';

export type Props = {
    post: Model.Post
};

export function Post(props: Props) {
    const post = props.post;

    const postedAt = moment(post.posted_at).format('YYYY-MM-DD HH:mm:ssZ');
    const updatedAt = post.updated_at && moment(post.updated_at).format('YYYY-MM-DD HH:mm:ssZ');
    return <article className="article">
        <header className="header"><Link to={`/posts/${post.id}`}>■</Link></header>
        <section className="body" dangerouslySetInnerHTML={{__html: post.body}} />
        {updatedAt && <div className="updated-at">Updated: {updatedAt}</div>}
        <div className="posted-at">{postedAt}</div>
    </article>;
}
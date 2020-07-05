import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { useState, useEffect } from 'react';
import { fetchAdminApi, PostsResponse } from '../../func/api';
import { RouteComponentProps } from '@reach/router';

type Props = RouteComponentProps & {
    id?: number
};

export function Edit(props: Props) {
    const [id, setId] = useState(0);
    const [body, setBody] = useState('');

    useEffect(() => {
        fetchAdminApi<PostsResponse>(`/posts/${props.id}`)
            .then((resp) => {
                const post = resp.posts[0];
                setId(post.id);
                setBody(post.body);
            })
    }, []);

    function update() {
        console.log(id);
        console.log(body);
    }

    return <div className="edit">
        <textarea value={body} onChange={(e) => setBody(e.currentTarget.value)} />
        <input type="submit" value="Submit" onClick={(e) => update()} />
    </div>;
}
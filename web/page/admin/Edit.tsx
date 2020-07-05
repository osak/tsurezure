import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { useState, useEffect } from 'react';
import { fetchAdminApi, AdminGetPostResponse } from '../../func/api';
import { RouteComponentProps } from '@reach/router';

type Props = RouteComponentProps & {
    id?: number
};

export function Edit(props: Props) {
    const [id, setId] = useState(0);
    const [body, setBody] = useState('');
    const [message, setMessage] = useState('');

    useEffect(() => {
        fetchAdminApi<AdminGetPostResponse>(`/posts/${props.id}`)
            .then((resp) => {
                const post = resp.post;
                setId(post.id);
                setBody(post.body);
            })
    }, []);

    function update() {
        const payload = {
            id,
            body,
        }
        fetchAdminApi(`/posts/${props.id}`, {method: 'PUT', body: JSON.stringify(payload)})
            .then((resp) => setMessage("ok"))
            .catch((e) => {
                console.error(e);
                setMessage(e)
            });
    }

    return <div className="edit">
        <textarea value={body} onChange={(e) => setBody(e.currentTarget.value)} />
        <input type="submit" value="Submit" onClick={(e) => update()} />
        <div className="edit__message">{message}</div>
    </div>;
}
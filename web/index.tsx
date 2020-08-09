import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Router, Link } from '@reach/router';
import { Main } from './page/main/Main';
import { Single } from './page/single/Single';

function Tsurezure() {
    return <div className="main">
        <h1 id="title"><Link to="/" className="title--link">徒然</Link></h1>
        <Router>
            <Main path="/" />
            <Single path="/posts/:id" />
        </Router>
    </div>;
}

ReactDOM.render(
    <Tsurezure />,
    document.getElementById('main')
);

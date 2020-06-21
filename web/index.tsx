import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Router } from '@reach/router';
import { Main } from './page/main/Main';

ReactDOM.render(
    <Router>
        <Main path="/" />
    </Router>,
    document.getElementById('main')
);

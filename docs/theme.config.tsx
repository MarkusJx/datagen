import React from 'react';
import { DocsThemeConfig } from 'nextra-theme-docs';

const config: DocsThemeConfig = {
    logo: <span>datagen</span>,
    project: {
        link: 'https://github.com/MarkusJx/datagen',
    },
    docsRepositoryBase: 'https://github.com/MarkusJx/datagen/docs',
    footer: {
        text: (
            <span>
                MIT {new Date().getFullYear()} ©{' '}
                <a href="https://github.com/MarkusJx/datagen" target="_blank">
                    MarkusJx
                </a>
                .
            </span>
        ),
    },
    toc: {
        backToTop: true
    },
    useNextSeoProps() {
        return {
            titleTemplate: '%s | datagen',
        };
    },
};

export default config;

import Vue from 'vue';

import './base.js';
import '../css/admin.css';

const Plotly = require('plotly.js-dist');
const axios = require('axios');


document.addEventListener('DOMContentLoaded', function() {
    var vm = new Vue({
        el: '#app',
        delimiters: ['[[',']]'],
        data: {
            error: false,
            computing: false,
            plots: [],
        },
        methods: {
            fetchData: function() {
                this.computing = true;
                axios({
                    method: 'GET',
                    url: Flask.url_for('admin.get_data'),
                })
                .then(resp => {
                    this.plots = resp.data;

                    this.$nextTick(() => {
                        this.plots.forEach(plot => {
                            Plotly.newPlot(plot.id, JSON.parse(plot.data));
                        });
                    });
                })
                .catch(err => {
                    this.error = true;
                })
                .then(() => {
                    this.computing = false;
                });
            },
        },
        created:  function () {
            this.fetchData();
        },
    });
});

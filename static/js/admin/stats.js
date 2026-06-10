(function() {
    const data = JSON.parse(document.getElementById('stats-data').textContent);

    if (data.jours) {
        const joursLabels = data.jours.map(j => j.date);
        const joursCA     = data.jours.map(j => j.ca);
        const joursNb     = data.jours.map(j => j.nb);
        new Chart(document.getElementById('chartJours'), {
            type: 'bar',
            data: {
                labels: joursLabels,
                datasets: [
                    {
                        label: 'CA (€)',
                        data: joursCA,
                        backgroundColor: 'rgba(59,130,246,.7)',
                        yAxisID: 'yCA',
                        order: 2,
                    },
                    {
                        label: 'Nb commandes',
                        data: joursNb,
                        type: 'line',
                        borderColor: '#f59e0b',
                        backgroundColor: 'transparent',
                        pointBackgroundColor: '#f59e0b',
                        tension: .3,
                        yAxisID: 'yNb',
                        order: 1,
                    },
                ],
            },
            options: {
                responsive: true,
                scales: {
                    yCA: { position: 'left',  title: { display: true, text: 'CA €' } },
                    yNb: { position: 'right', title: { display: true, text: 'Commandes' }, grid: { drawOnChartArea: false } },
                },
            },
        });
    }

    new Chart(document.getElementById('chartTypes'), {
        type: 'doughnut',
        data: {
            labels: ['Carte', 'Traiteur'],
            datasets: [{
                data: [data.nb_carte, data.nb_traiteur],
                backgroundColor: ['#3b82f6', '#f59e0b'],
            }],
        },
        options: { responsive: true, plugins: { legend: { position: 'bottom' } } },
    });
})();

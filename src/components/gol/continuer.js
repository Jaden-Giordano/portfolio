let stagList = [{
        search: [
            [0, 0, 0],
            [0, 1, 0],
            [1, 0, 1],
            [1, 0, 1],
            [0, 1, 0],
            [0, 0, 0],
        ],
        replace: [
            [0, 0, 0],
            [0, 1, 0],
            [1, 1, 1],
            [1, 0, 1],
            [0, 1, 0],
            [0, 0, 0],
        ]
    },
    {
        search: [
            [0, 0, 1, 1, 0, 0],
            [0, 1, 0, 0, 1, 0],
            [0, 0, 1, 1, 0, 0]
        ],
        replace: [
            [0, 0, 1, 1, 0, 0],
            [0, 1, 1, 0, 1, 0],
            [0, 0, 1, 1, 0, 0]
        ]
    },
    {
        search: [
            [0, 0, 0],
            [0, 1, 0],
            [0, 1, 0],
            [0, 1, 0],
            [0, 0, 0],
        ],
        replace: [
            [0, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
            [0, 1, 0],
            [0, 0, 0],
        ]
    },

    {
        search: [
            [0, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        replace: [
            [0, 0, 0, 0],
            [0, 0, 0, 1],
            [0, 1, 0, 1],
            [0, 0, 1, 1]

        ]
    }
]


function clearStags(cells) {
    for (let i = 0; i < cells.length; i++) {
        for (let j = 0; j < cells[i].length; j++) {
            for (let m = 0; m < stagList.length; m++) {
                let matches = 0;
                let s = stagList[m];
                //search for stagnant shapes at current location
                // facing ^
                for (let k = 0; k < s.search.length; k++) {
                    for (let l = 0; l < s.search[k].length; l++) {
                        if (typeof cells[i + k] != 'undefined') {
                            if (typeof cells[i + k][j + l] != 'undefined') {
                                if (Math.floor(cells[i + k][j + l] / 10) == s.search[k][l]) {
                                    matches++;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }

                //facing V
                if (matches == s.search.length * s.search[0].length) {
                    for (let k = 0; k < s.replace.length; k++) {
                        for (let l = 0; l < s.replace[k].length; l++) {
                            cells[i + k][j + l] = s.replace[k][l] * 10;
                        }
                    }
                }

            }
        }
    }
    return cells;
}

module.exports = clearStags;
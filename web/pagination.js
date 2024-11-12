
const resultsPerPage = 5;
let currentPage = 1;
let results = [];
const initialNews = [
    { 
        id: 1, 
        title: "Sanders back in U.S. Senate, blasts 'colonialism' in Puerto Rico", 
        fakeOrNot: false, 
        articleText: "WASHINGTON (Reuters) - Democratic U.S. presidential hopeful Bernie Sanders brought his firebrand rhetoric back to the floor of the Senate on Tuesday to condemn a White House-backed bill on Puerto Rico’s financial crisis as 'colonialism at its worst.' Sanders, a self-described democratic socialist who turned an unlikely presidential bid into a political movement to combat inequality, warned that legislation due for a crucial Senate vote on Wednesday would subject Puerto Rico to Republican trickle-down economics and favor 'vulture capitalists' at the expense of the island’s increasingly impoverished population. An aide said it was the first time Sanders has spoken in the Senate since December. 'Does that sound like the kind of morality that should be passed here in the United States Senate?' Sanders fumed during an eight-minute appearance to support an hours-long speech by Democratic Senator Robert Menendez of New Jersey, who opposes the bill. The Vermont senator used his appearance to rail against details of the legislation, which would put much of Puerto Rico’s management in the hands of a seven-member oversight board and require the island to pay $370 million over five years for the board’s administration costs even as it cuts funding for education, healthcare and pensions. In an exchange with Menendez, Sanders said, 'How in God’s name do you run up an administrative cost of $370 million,' adding, 'I know this sounds so absurd that people may think I’m misleading them.' 'Would my friend from New Jersey agree that this is colonialism at its worst?' asked Sanders, who announced last week that he would vote for presumptive Democratic presidential nominee Hillary Clinton in her November election contest against Republican Donald Trump. 'Oh absolutely,' replied Menendez. The Puerto Rico bill, which has already been passed by the Republican-controlled House of Representatives, faces opposition from both sides of the aisle in the Senate. Nevertheless, the Obama administration and Republican leaders are trying to get the bill passed before a July 1 deadline, when the island is scheduled to make a $1.9 billion payment on its $70 billion debt."
    },
    { 
        id: 2, 
        title: "Kremlin: Syria peoples' congress being 'actively discussed'", 
        fakeOrNot: false, 
        articleText: "MOSCOW (Reuters) - A proposal to convene a congress of all Syria’s ethnic groups is a joint initiative which is being promoted by Russia and others and is now being actively discussed, the Kremlin said on Friday. It is premature, however, to discuss the time and venue for the congress, which is seen as a mechanism to assist Syria’s post-war development, Putin’s spokesman Dmitry Peskov told a conference call with reporters. Putin mentioned the idea of holding such a congress at a forum with foreign scholars on Thursday."
    },
    { 
        id: 3, 
        title: "Oregon Cop Convicted Of Shattering Biker’s Collarbone With Kick Forgot Dashcam Was On (VIDEO)", 
        fakeOrNot: true, 
        articleText: "In a baffling fit of rage, an Oregon State Police officer has been convicted of ramming his car into a stopped motorcyclist then kicking the dazed man so hard in the chest that he would need a metal plate and screws to repair his shattered collarbone and ribs. What State Police Captain Rob Edwards didn’t know was that his department had installed a functioning dashboard camera. It captured the entire shocking scene. The 2012 incident started with a chase that the victim says he didn’t know was happening. The first he learned of Edwards' presence was when the cop was running directly into the back of his motorcycle, spilling the man from his bike. The man gets up with his hands raised only to find Edwards rushing at him with his pistol held sideways, probably imitating something he saw in a movie. The biker, identified as Justin Wilkens, barely had time to say 'What did I do?' before Edwards kicked him square in the chest with such force that Wilkens would later learn he broke his collarbone and several ribs. Edwards would later say that he was 'frustrated' with Wilkens for not complying with his demands sooner, and that he didn’t realize his car was equipped with a functioning dashcam. When Edwards eventually struck Wilkens' motorcycle with his car, then pulled his gun and kicked Wilkens in the chest, he didn’t know the whole episode would be played again and again before a U.S. District Court jury. It’s not hard to see why the dashcam was a bitter pill for Edwards; thanks to the footage, his trial lasted just three days, and he was promptly convicted. He and his department will now have to pay $180,000 in damages."
    },
    { 
        id: 4, 
        title: "Twitter Erupts With Glee Over #CruzSexScandal Rumors (TWEETS)", 
        fakeOrNot: true,  
        articleText: "The last thing any politician running for the presidency needs is negative or scandalous hashtags about them trending on Twitter. However, that is just what America is waking up with regards to GOP presidential hopeful Ted Cruz. Overnight, rumors began circulating that Ted Cruz has been cheating on his wife with MULTIPLE women. Of course, these are just rumors at this time, apparently started by The National Enquirer, and there is certainly no proof. But in the age of the internet, that doesn’t matter. Ted Cruz is a polarizing figure, and people on both the left and right are having a field day over this one."
    },
    { 
        id: 5, 
        title: "MUST WATCH VIDEO: Obama Tries To Trash Trump But Turns Into A Babbling Mess [Video]", 
        fakeOrNot: true, 
        articleText: "This is too good to miss! Mr. Teleprompter didn’t do so well when he went off script during an appearance in Indiana."
    }
];


document.querySelectorAll('input[name="fakeornot"]').forEach(checkbox => {
    checkbox.addEventListener("change", applyFilters);
});



document.getElementById("search-btn").addEventListener("click", function(event) {
    event.preventDefault();
    const query = document.getElementById("search-txt").value.trim();
    if (query) {
        fetchResults(query);
    } else {
        displayFilteredResults(initialNews); 
    }
});

function fetchResults(query) {
    const apiUrl = ` `;

    fetch(apiUrl)
        .then(response => response.json())
        .then(data => {
            results = data.data;
            const totalResults = results.length;

            if (totalResults === 0) {
                document.getElementById("result-count").textContent = "Total Results: 0";
                document.getElementById("results-container").innerHTML = "<div>No results found.</div>"; 
                const pagination = document.querySelector(".pagination");
                pagination.style.display = "none";
            } else {
                document.getElementById("result-count").textContent = `Total Results: ${totalResults}`;
                currentPage = 1; 
                displayPage(currentPage);
                togglePagination(totalResults);
            }
        })
        .catch(error => {
            console.error("Error fetching results:", error);
            document.getElementById("result-count").textContent = "Total Results: 0";
            document.getElementById("results-container").innerHTML = "<div</div>"; 
            results = [];
        });
}

function displayPage(page) {
    const resultsContainer = document.getElementById("results-container");
    resultsContainer.innerHTML = ""; 

    const start = (page - 1) * resultsPerPage;
    const end = start + resultsPerPage;
    const currentResults = results.slice(start, end);
    if (currentResults.length === 0) {
        return ;
    }

    currentResults.forEach(result => {
        const resultItem = document.createElement("div");
        resultItem.className = "result-item";
        resultItem.innerHTML = `
            <strong>${result.fakeOrNot ? 'Fake' : 'Real'}</strong><br>   
            <strong>${result.title}</strong><br>
        `;
        resultItem.dataset.id = result.id;
        resultsContainer.appendChild(resultItem);
    });

    document.getElementById("current-page").textContent = `Page ${page} of ${totalPages}`;
    setupPagination(totalPages);
}
function togglePagination(totalResults) {
    const pagination = document.querySelector(".pagination");
    pagination.style.display = totalResults > resultsPerPage ? "block" : "none";
}
function setupPagination(totalPages) {
    document.getElementById("prev-page").style.display = currentPage > 1 ? "inline" : "none";
    document.getElementById("next-page").style.display = currentPage < totalPages ? "inline" : "none";
}

document.getElementById("prev-page").addEventListener("click", function(event) {
    event.preventDefault();
    if (currentPage > 1) {
        currentPage--;
        displayPage(currentPage);
    }
});

document.getElementById("next-page").addEventListener("click", function(event) {
    event.preventDefault();
    const totalPages = Math.ceil(results.length / resultsPerPage);
    if (currentPage < totalPages) {
        currentPage++;
        displayPage(currentPage);
    }
});
function displayInitialNews() {
    results = initialNews; 
    const totalResults = results.length;
    document.getElementById("result-count").textContent ="";
    currentPage = 1; 
    displayPage(currentPage);
    const pagination = document.querySelector(".pagination");
    pagination.style.display = totalResults > 0 ? "block" : "none"; 
}

function applyFilters() {
    const fakeCheckbox = document.querySelector('input[value="fake"]').checked;
    const realCheckbox = document.querySelector('input[value="real"]').checked;

    if (results.length === 0) {
        document.getElementById("result-count").textContent = "Total Result: 0";
        document.getElementById("results-container").innerHTML = "<div></div>";
        return;
    }

    const filteredResults = results.filter(article => {
        if (fakeCheckbox && article.fakeOrNot === true) return true;
        if (realCheckbox && article.fakeOrNot === false) return true;
        return false;
    });

    if (!fakeCheckbox && !realCheckbox) {
        results = initialNews;
    } else {
        results = filteredResults;
    }

    currentPage = 1; 
    const totalResults = results.length;
    document.getElementById("result-count").textContent = `Total Result: ${totalResults}`;
    displayPage(currentPage);
    togglePagination(totalResults);
}

document.addEventListener("DOMContentLoaded", function() {
    displayInitialNews(); 
});

const modal = document.getElementById("popup-modal");
const closeButton = document.querySelector(".close-btn");
const modalCloseButton = document.getElementById("modal-close-button");

function showModal(result) {
    const modalTitle= document.getElementById("modal-title");
    modalTitle.innerHTML = `
        <strong>Title:</strong> ${result.title}<br>
    `;
    const modalMessage = document.getElementById("modal-message");
    modalMessage.innerHTML = `
        <strong></strong> ${result.articleText}<br>
        <strong>Fake or Not:</strong> ${result.fakeOrNot ? "Fake" : "Not Fake"}<br>
        <strong>Similarity Score:</strong> ${result.similarityScore}
    `;
    modal.style.display = "flex";
}

function closeModal() {
    modal.style.display = "none";
}

closeButton.addEventListener("click", closeModal);
modalCloseButton.addEventListener("click", closeModal);
window.addEventListener("click", function(event) {
    if (event.target === modal) {
        closeModal();
    }
});

document.getElementById("results-container").addEventListener("click", function(event) {
    if (event.target.classList.contains("result-item")) {
        const resultId = event.target.dataset.id;
        const selectedResult = results.find(result => result.id == resultId);
        if (selectedResult) {
            showModal(selectedResult);
        }
    }
});
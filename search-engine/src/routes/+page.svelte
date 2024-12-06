<style>
    :global(body) {
        background: rgba(18, 18, 41, 0.8);
        height: 100vh; 
        width: 100vw; 
        margin: 0; 
    }
    
    header {
        background: rgba(31, 31, 71, 0.8);
        color: #ffffff;
        padding: 20px; 
        text-align: center; 
        height: 10vh;
        font-size: large;
    }
    
    #search-box {
        position: absolute;
        margin-top: 50px;
        margin-bottom: 20px;
        left: 50%;
        transform: translate(-50%, -50%);
        background: #2f3640;
        height: 40px; 
        border-radius: 60px;
        padding: 10px;
        display: flex;
        align-items: center; 
    }
    
    #search-box:hover > .search-txt,
    .search-txt:focus {
        width: 500px; 
        padding: 0 6px; 
    }
    
    #search-box:hover > .search-btn {
        background: white;
    }
    
    #search-btn {
        color: #988fc7;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background: #2f3640;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.4s;
    }
    
    #search-txt {
        border: none;
        background: none;
        outline: none;
        color: white;
        font-size: 16px;
        transition: 0.4s;
        line-height: 40px;
        width: 400px; 
    }
    
    .container {
        display: flex;
        margin-top: 90px;
        margin-left: 20px;
        margin-right: 20px;
    }
    
    .filter-section {
        flex: 1;
        padding: 20px;
    
    }
    
    .filter-box { 
        border: 1px solid #2f3640; 
        border-radius: 5px; 
        margin-bottom: 20px; 
        box-shadow: 0px 2px 5px rgba(0, 0, 0, 0.1); 
        padding: 20px;
    }
    
    .filter-box:hover {
        transform: scale(1.05); 
    }
    
    .filter-box h2{
        margin-top: 2px;
        margin-bottom: 15px;
        font-size: 25px;
        color: #ffffff;
    }
    .filter-box label {
        font-size: 20px;
        margin-bottom: 1px;
        color: #ffffff;
    
    }
    
    .results-section {
        flex: 3; 
        padding: 20px;
    }
    
    .result-item {
        padding: 10px;
        border: 1px solid #2f3640;
        margin-bottom: 10px;
        border-radius: 5px;
        color: #ffffff;
    }
    
    .result-item:hover{
        transform: scale(1.05);
    }
    
    .result-info {
        display: flex;
        justify-content: center; 
        margin-bottom: 10px;
        color: #ffffff;
    }
    
    .result-info div {
        margin-left: 20px; 
    }
    
    .pagination {
        position: absolute;
        right: 40px;
       
    }
    
    .modal {
        display: flex;
        justify-content: center;
        align-items: center;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);    
    }
    
    .modal-content {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: #2f3640;
        padding: 20px;
        border-radius: 8px;
        width:1000px;
        color: #ffffff;
        text-align: center;
        box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.2);
        max-height: 70%; 
        width: 90%;
        overflow-y: auto;
        align-items: center;
    }
    
    #modal-close-button {
        background-color: #988fc7;
        color: #ffffff;
        padding: 10px 20px;
        border: none;
        border-radius: 5px;
        cursor: pointer;
        margin-top: 15px;
        
    }
    
    #modal-close-button:hover {
        background-color: #7968a0;
    }
    
    .similar-docs {
        margin-top: 15px;
        padding-top: 10px;
        border-top: 1px solid #ddd;
        font-size: 0.9em;
        text-align: center;
    }
    .similar-docs {
        margin-top: 5px;
    }
    .clickable {
        all: unset;
        cursor: pointer; 
        padding: 10px; 
        border: 1px solid transparent; 
        border-radius: 5px;
        transition: background-color 0.2s, border-color 0.2s; 
        text-decoration: underline; 

      }
    
    </style>
    
    <script>
        import SimilarDocumentModal from './similardoc.svelte';
        import { onMount } from "svelte";
        const API_URL=" ";
        let modalVisible = false;
        let search = '';
        let results = [];
        let currentResults = []; 
        let currentPage = 1;
        let resultsPerPage = 5;
        let filteredResults = [];
        let totalPages = 1;
        let showResultsInfo = false;
        let fakeChecked = false;
        let realChecked = false;
        let totalResults=0;
        let selectedResult = {};
        let similarModalVisible = false;
        let selectedSimilarDocument = {};
        let initialNews = [
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
    
        onMount(() => {
            const script = document.createElement("script");
            script.src = "https://use.fontawesome.com/releases/v5.0.6/js/all.js";
            script.defer = true;
            document.head.appendChild(script);
            displayInitialNews();
        });
    
        function displayInitialNews() {
            currentResults = initialNews;
            results=currentResults;
        }
    
        async function formSubmitted(event) {
            event.preventDefault();
            const url = `${API_URL}${search}`;
            response = await fetchResults(url);
            const json = await response.json();
            results = json.data.map(news => news.url);
            currentResults = results;
            showResultsInfo=true;
            totalPages= Math.ceil(results.length / resultsPerPage);
        }
    
        function applyFilters() {
        filteredResults = results.filter(article => {
            if (fakeChecked && !realChecked) {
                return article.fakeOrNot === true; 
            } else if (!fakeChecked && realChecked) {
                return article.fakeOrNot === false; 
            } else {
                return true; 
            }
        });
        showResultsInfo = true;
        totalResults = filteredResults.length;
    }  
    
        $: currentResults = filteredResults;
        
        function prevPage() {
            if (currentPage > 1) {
                currentPage--;
                displayPage(currentPage);
            }
        }
    
        function nextPage() {
            if (currentPage < totalPages) {
                currentPage++;
                displayPage(currentPage);
            }
        }
    
        function showModal(result) {
            selectedResult = result;
            modalVisible = true;
        }
    
        function closeModal() {
            modalVisible = false;
            selectedResult = null; 
        }
    
        function showSimilarModal(doc) {
            selectedSimilarDocument = doc;
            similarModalVisible = true;
        }
    
        function closeSimilarModal() {
            similarModalVisible = false;
        }
    
    
        function findSimilarDocuments(result) {
    
            return results.filter(doc => doc.id !== result.id && doc.fakeOrNot === result.fakeOrNot);
        }
    </script>
    <main>
        <header>
            <h1>News SearchWeb</h1>
        </header>
        <form id="search-box">
            <input bind:value={search} id="search-txt" class="search-txt" type="text" placeholder="What are you looking for?">
            <button  on:click={formSubmitted} id="search-btn" class="search-btn" aria-label="Search">
                <i class="fas fa-search"></i>
            </button>
        </form>
        <div class="container">
            <div class="filter-section">
                <div class="filter-box">
                    <h2>Fake Or Not</h2>
                    <label>
                        <input type="checkbox" class="checkbox" id="fake" name="fakeornot" value="fake" bind:checked={fakeChecked} on:change={applyFilters}> Fake
                    </label><br>
                    <label>
                        <input type="checkbox" id="real" name="fakeornot" value="real" bind:checked={realChecked} on:change={applyFilters}> Real
                    </label><br>
                </div>
            </div>
            
            <div class="results-section">
                {#if showResultsInfo}
                    <div class="result-info">
                        <div id="result-count">Results: {totalResults}</div>
                        <div id="current-page">Page {currentPage} of {totalPages}</div>
                        <div class="pagination" style="display: none;">
                            {#if currentPage > 1}
                                <button on:click={prevPage} id="prev-page" aria-label="Previous Page">&lt;&lt;</button>
                            {/if}
                            {#if currentPage < totalPages}
                                <button on:click={nextPage} id="next-page" aria-label="Next Page">&gt;&gt;</button>
                            {/if}
                        </div>
                    </div>
                {/if}
                <div id="results-container">
    
                    {#each currentResults as result}
                        <div class="result-item"
                        role="button"
                        tabindex="0"
                        aria-label={`Details about ${result.title}`}
                        on:click={() => showModal(result)}
                        on:keydown={(event) => {
                          if (event.key === 'Enter') {
                            showModal(result);
                          }
                        }}
                        data-id={result.id}>
                            <strong>{result.fakeOrNot ? 'Fake' : 'Real'}</strong><br>
                            <strong>{result.title}</strong><br>
                        </div>
                    {/each}
                </div>
                {#if modalVisible}
                    <div id="popup-modal" class="modal">
                        <div class="modal-content">
                            <h2 id="modal-title"><strong>Title:</strong> {selectedResult.title}</h2>
                            <div id="modal-message" style="text-align: left;">
                                <strong></strong> {selectedResult.articleText}<br><br>
                                <strong>Fake or Not:</strong> {selectedResult.fakeOrNot ? "Fake" : "Not Fake"}<br><br>
                                <strong>Similarity Score:</strong> {selectedResult.similarityScore}<br><br>
                                <strong>Similar Document:</strong>
                                {#if findSimilarDocuments(selectedResult).length > 0}
                                <div class="similar-docs">
                                  {#each findSimilarDocuments(selectedResult) as doc}
                                    <button
                                    class="clickable"
                                    on:click={() => showSimilarModal(doc)}
                                    on:keydown={(e) => e.key === 'Enter' && handleClick()}
                                    data-id={doc.id}
                                    >
                                    - {doc.title}
                                    </button>
                                  {/each}
                                </div>
                                {:else}
                                    <div>Can't find similar documents.</div>
                                {/if}
                            </div>
                            <button id="modal-close-button" on:click={closeModal}>Close</button>
                        </div>
                    </div>
                {/if}
                <SimilarDocumentModal visible={similarModalVisible} document={selectedSimilarDocument} close={closeSimilarModal} />
            </div>
        </div>
    </main>
    
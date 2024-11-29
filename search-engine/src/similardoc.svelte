<script>
    export let visible = false;
    export let document = {};
    export let close;
</script>
  
  {#if visible}
    <div class="modal">
      <div class="modal-content">
        <h2 id="modal-title"><strong>Title:</strong> {document.title}</h2>
        <div id="modal-message" style="text-align: left;">
          <strong>Content:</strong> {document.articleText}<br><br>
          <strong>Fake or Not:</strong> {document.fakeOrNot ? "Fake" : "Not Fake"}<br><br>
          <strong>Similarity Score:</strong> {document.similarityScore}<br><br>
        </div>
        <button id="modal-close-button" on:click={close}>Close</button>
      </div>
    </div>
  {/if}
  
<style>
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
</style>
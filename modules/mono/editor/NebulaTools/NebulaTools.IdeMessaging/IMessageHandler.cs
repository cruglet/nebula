using System.Threading.Tasks;

namespace NebulaTools.IdeMessaging
{
    public interface IMessageHandler
    {
        Task<MessageContent> HandleRequest(Peer peer, string id, MessageContent content, ILogger logger);
    }
}
